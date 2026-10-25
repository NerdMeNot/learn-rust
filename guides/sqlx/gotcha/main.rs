use sqlx::FromRow;
use sqlx::sqlite::SqlitePoolOptions;

#[derive(FromRow)]
struct Note {
    id: i64,
    text: String,
    done: bool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // An in-memory SQLite database, so there is nothing to set up or clean up.
    // For a file, see SqliteConnectOptions (it has create_if_missing).
    let pool = SqlitePoolOptions::new().connect("sqlite::memory:").await?;

    sqlx::query(
        "CREATE TABLE notes (
            id INTEGER PRIMARY KEY,
            text TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await?;

    for text in ["learn ownership", "write an axum service"] {
        sqlx::query("INSERT INTO notes (text) VALUES ($1)")
            .bind(text) // the value travels separately from the SQL text
            .execute(&pool)
            .await?;
    }
    sqlx::query("UPDATE notes SET done = 1 WHERE id = $1")
        .bind(1)
        .execute(&pool)
        .await?;

    let notes: Vec<Note> = sqlx::query_as!(Note, "SELECT id, text, done FROM notes ORDER BY id")
        .fetch_all(&pool)
        .await?;
    for note in &notes {
        let mark = if note.done { "x" } else { " " };
        println!("[{mark}] {} {}", note.id, note.text);
    }

    let open: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM notes WHERE done = 0")
        .fetch_one(&pool)
        .await?;
    println!("{open} still open");
    Ok(())
}
