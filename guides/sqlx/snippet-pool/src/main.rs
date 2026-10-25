use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(3)
        .connect("sqlite::memory:")
        .await?;
    sqlx::query("CREATE TABLE notes (id INTEGER PRIMARY KEY, text TEXT NOT NULL)")
        .execute(&pool)
        .await?;

    // Hold three connections at once, so each of them is a different one.
    let mut a = pool.acquire().await?;
    let mut b = pool.acquire().await?;
    let mut c = pool.acquire().await?;
    for (name, conn) in [("a", &mut a), ("b", &mut b), ("c", &mut c)] {
        let found: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE name = 'notes'")
            .fetch_one(&mut **conn)
            .await?;
        println!("connection {name} sees the notes table: {}", found == 1);
    }
    Ok(())
}
