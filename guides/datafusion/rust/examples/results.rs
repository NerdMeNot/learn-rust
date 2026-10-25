use datafusion::arrow::array::AsArray;
use datafusion::arrow::datatypes::Float64Type;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    // A CSV file this time: DataFusion reads the header and guesses the column types.
    let path = std::env::temp_dir().join("sales.csv");
    std::fs::write(
        &path,
        "city,month,sales\nPune,1,120.0\nPune,2,135.5\nDelhi,1,210.0\nDelhi,2,190.25\nChennai,1,99.0\n",
    )?;

    let ctx = SessionContext::new();
    ctx.register_csv("sales", path.to_str().unwrap(), CsvReadOptions::new())
        .await?;

    for field in ctx.table("sales").await?.schema().fields() {
        println!("column {} has type {}", field.name(), field.data_type());
    }

    // collect() gives the result back as Arrow record batches, so you can use it in code.
    let batches = ctx
        .sql("SELECT city, SUM(sales) AS total FROM sales GROUP BY city ORDER BY total DESC")
        .await?
        .collect()
        .await?;
    for batch in &batches {
        let city = batch.column_by_name("city").unwrap().as_string::<i32>();
        let total = batch
            .column_by_name("total")
            .unwrap()
            .as_primitive::<Float64Type>();
        for i in 0..batch.num_rows() {
            println!("{} -> {:.2}", city.value(i), total.value(i));
        }
    }

    // Mistakes come back as errors, not panics.
    for sql in [
        "SELECT * FROM sale",
        "SELECT cty FROM sales",
        "SELEC city FROM sales",
    ] {
        match ctx.sql(sql).await {
            Ok(_) => println!("{sql}: ok"),
            Err(e) => println!("{sql}\n  Error: {e}"),
        }
    }
    Ok(())
}
