use sqlx::{AssertSqlSafe, SqlitePool};
use sqlx::sqlite::SqlitePoolOptions;

// What a user typed into a form.
const INPUT: &str = "x'); DELETE FROM notes; --";

async fn fresh() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1) // not required: pooled in-memory connections share one database (task 1)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query("CREATE TABLE notes (id INTEGER PRIMARY KEY, text TEXT NOT NULL)")
        .execute(&pool)
        .await?;
    for text in ["learn ownership", "write an axum service"] {
        sqlx::query("INSERT INTO notes (text) VALUES ($1)")
            .bind(text)
            .execute(&pool)
            .await?;
    }
    Ok(pool)
}

async fn count(pool: &SqlitePool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar("SELECT COUNT(*) FROM notes").fetch_one(pool).await
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Pasting the input into the SQL text. AssertSqlSafe is how you tell sqlx you meant it.
    let pool = fresh().await?;
    let sql = format!("INSERT INTO notes (text) VALUES ('{INPUT}')");
    sqlx::query(AssertSqlSafe(sql)).execute(&pool).await?;
    println!("pasted into the SQL: {} notes left", count(&pool).await?);

    // Binding it: the input is a value, whatever characters it contains.
    let pool = fresh().await?;
    sqlx::query("INSERT INTO notes (text) VALUES ($1)")
        .bind(INPUT)
        .execute(&pool)
        .await?;
    println!("bound: {} notes, the last one is {:?}", count(&pool).await?,
        sqlx::query_scalar::<_, String>("SELECT text FROM notes ORDER BY id DESC LIMIT 1").fetch_one(&pool).await?);
    Ok(())
}
