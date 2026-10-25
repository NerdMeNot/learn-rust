use sqlx::FromRow;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(FromRow)]
struct Note {
    id: i64,
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
    sqlx::query("CREATE TABLE notes (id INTEGER PRIMARY KEY, text TEXT NOT NULL)")
        .execute(&pool)
        .await?;
    sqlx::query("INSERT INTO notes (text) VALUES ($1)")
        .bind("learn ownership")
        .execute(&pool)
        .await?;

    // The struct field is `text`; this query names the column `body`.
    let notes: Vec<Note> = sqlx::query_as("SELECT id, text AS body FROM notes")
        .fetch_all(&pool)
        .await?;
    println!("{}: {}", notes[0].id, notes[0].text);
    Ok(())
}
