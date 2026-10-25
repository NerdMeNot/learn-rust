use std::sync::Arc;

use datafusion::arrow::array::{Float64Array, Int64Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::parquet::arrow::ArrowWriter;
use datafusion::parquet::file::properties::WriterProperties;
use datafusion::physical_plan::{ExecutionPlan, collect, displayable};
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // The same five rows as the main program, but written two rows per Parquet row group.
    let schema = Arc::new(Schema::new(vec![
        Field::new("city", DataType::Utf8, false),
        Field::new("month", DataType::Int64, false),
        Field::new("sales", DataType::Float64, false),
    ]));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(vec![
                "Pune", "Pune", "Delhi", "Delhi", "Chennai",
            ])),
            Arc::new(Int64Array::from(vec![1, 2, 1, 2, 1])),
            Arc::new(Float64Array::from(vec![120.0, 135.5, 210.0, 190.25, 99.0])),
        ],
    )?;
    let path = std::env::temp_dir().join("sales_groups.parquet");
    let props = WriterProperties::builder()
        .set_max_row_group_row_count(Some(2))
        .build();
    let mut writer = ArrowWriter::try_new(std::fs::File::create(&path)?, schema, Some(props))?;
    writer.write(&batch)?;
    writer.close()?;

    // One partition, so the plan does not depend on how many CPU cores this machine has
    // (the default is one partition per core).
    let ctx = SessionContext::new_with_config(SessionConfig::new().with_target_partitions(1));
    ctx.register_parquet(
        "sales",
        path.to_str().unwrap(),
        ParquetReadOptions::default(),
    )
    .await?;

    // The same question asked two ways.
    // `cargo run --example explain -- 100` changes the number (200 if you give none).
    let limit: f64 = std::env::args()
        .nth(1)
        .map_or(200.0, |a| a.parse().expect("a number"));
    let from_sql = ctx
        .sql(&format!(
            "SELECT city, sales FROM sales WHERE sales > {limit}"
        ))
        .await?;
    let from_api = ctx
        .table("sales")
        .await?
        .filter(col("sales").gt(lit(limit)))?
        .select(vec![col("city"), col("sales")])?;
    println!(
        "-- as written (SQL)\n{}",
        from_sql.logical_plan().display_indent()
    );
    println!(
        "-- as written (DataFrame)\n{}",
        from_api.logical_plan().display_indent()
    );

    // The optimizer rewrites both. The unoptimized plans differ (Int64(200) against
    // Float64(200)); the optimized ones are equal.
    let sql_plan = from_sql.clone().into_optimized_plan()?;
    let api_plan = from_api.into_optimized_plan()?;
    println!("-- optimized\n{}", sql_plan.display_indent());
    println!("optimized plans equal: {}", sql_plan == api_plan);

    // Run the physical plan, then ask it what the Parquet reader did.
    let physical = from_sql.create_physical_plan().await?;
    // The plan prints the temp folder of the machine that runs it; show a placeholder
    // instead, so the page does not depend on that path (the plan drops the leading `/`).
    let shown = displayable(physical.as_ref()).indent(false).to_string().replace(
        path.to_str().unwrap().trim_start_matches('/'),
        "<temp dir>/sales_groups.parquet",
    );
    println!("-- physical\n{shown}");
    let rows: usize = collect(physical.clone(), ctx.task_ctx())
        .await?
        .iter()
        .map(|b| b.num_rows())
        .sum();
    println!("rows returned: {rows}");
    print_metrics(physical.as_ref());
    Ok(())
}

fn print_metrics(node: &dyn ExecutionPlan) {
    if let Some(v) = node
        .metrics()
        .and_then(|set| set.sum_by_name("row_groups_pruned_statistics"))
    {
        println!("{}: row_groups_pruned_statistics = {v}", node.name());
    }
    for child in node.children() {
        print_metrics(child.as_ref());
    }
}
