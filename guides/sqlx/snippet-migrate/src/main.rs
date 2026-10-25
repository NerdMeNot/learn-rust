use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1) // not required: pooled in-memory connections share one database (task 1)
        .connect("sqlite::memory:")
        .await?;

    // Reads ./migrations while compiling and embeds the files in the program.
    sqlx::migrate!().run(&pool).await?;
    // Running again applies nothing new.
    sqlx::migrate!().run(&pool).await?;

    let rows: Vec<(i64, String, bool)> =
        sqlx::query_as("SELECT version, description, success FROM _sqlx_migrations")
            .fetch_all(&pool)
            .await?;
    for (version, description, success) in rows {
        println!("migration {version}: {description} (success: {success})");
    }

    sqlx::query("INSERT INTO notes (text) VALUES ($1)")
        .bind("learn migrations")
        .execute(&pool)
        .await?;
    let n: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notes").fetch_one(&pool).await?;
    println!("{n} note");
    Ok(())
}
