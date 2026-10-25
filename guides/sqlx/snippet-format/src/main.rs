use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
    let text = "hello";
    let sql = format!("INSERT INTO notes (text) VALUES ('{text}')");
    sqlx::query(sql).execute(&pool).await?;
    Ok(())
}
