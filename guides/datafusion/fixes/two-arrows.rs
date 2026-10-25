use std::sync::Arc;

use arrow::array::Int64Array;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;

#[tokio::main]
async fn main() -> datafusion::error::Result<()> {
    let schema = Arc::new(Schema::new(vec![Field::new("month", DataType::Int64, false)]));
    let batch = RecordBatch::try_new(schema, vec![Arc::new(Int64Array::from(vec![1, 2]))]).unwrap();

    let ctx = SessionContext::new();
    ctx.register_batch("months", batch)?;
    ctx.sql("SELECT month FROM months").await?.show().await?;
    Ok(())
}
