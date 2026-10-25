use std::sync::Arc;

use datafusion::arrow::array::{Float64Array, Int64Array, StringArray};
use datafusion::arrow::datatypes::{DataType, Field, Schema};
use datafusion::arrow::record_batch::RecordBatch;
use datafusion::parquet::arrow::ArrowWriter;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // 1. Build a small table in memory as Arrow columns, then save it as Parquet.
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
    let path = std::env::temp_dir().join("sales.parquet");
    let mut writer = ArrowWriter::try_new(std::fs::File::create(&path)?, schema, None)?;
    writer.write(&batch)?;
    writer.close()?;

    // 2. Register the file as a table and ask a question in SQL.
    let ctx = SessionContext::new();
    ctx.register_parquet(
        "sales",
        path.to_str().unwrap(),
        ParquetReadOptions::default(),
    )
    .await?;
    ctx.sql("SELECT city, SUM(sales) AS total FROM sales GROUP BY city ORDER BY total DESC")
        .await?
        .show()
        .await?;

    // 3. A similar question with the DataFrame API. Both queries are sorted, so the
    //    rows come out in the same order every time.
    ctx.table("sales")
        .await?
        .filter(col("month").eq(lit(2)))?
        .select(vec![col("city"), col("sales")])?
        .sort(vec![col("sales").sort(false, true)])?
        .show()
        .await?;
    Ok(())
}
