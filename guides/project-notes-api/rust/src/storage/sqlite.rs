use super::{NoteStore, StoreError};
use crate::domain::{Note, NoteText};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

/// Notes in SQLite. Cloning shares the pool.
#[derive(Clone)]
pub struct SqliteStore {
    pool: SqlitePool,
}

/// The row as the database has it. The domain's `Note` knows nothing about sqlx.
#[derive(sqlx::FromRow)]
struct NoteRow {
    id: i64,
    text: String,
}

impl From<NoteRow> for Note {
    fn from(row: NoteRow) -> Note {
        Note {
            id: row.id,
            text: row.text,
        }
    }
}

impl From<sqlx::Error> for StoreError {
    fn from(e: sqlx::Error) -> StoreError {
        StoreError::new(e)
    }
}

impl SqliteStore {
    /// Opens (or, with `mode=rwc`, creates) the database and applies `migrations/`.
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str(url)?;
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        // Reads ./migrations while compiling and embeds the files in the program.
        sqlx::migrate!().run(&pool).await?;
        Ok(SqliteStore { pool })
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}

impl NoteStore for SqliteStore {
    async fn list(&self) -> Result<Vec<Note>, StoreError> {
        let rows: Vec<NoteRow> = sqlx::query_as("SELECT id, text FROM notes ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Note::from).collect())
    }

    async fn get(&self, id: i64) -> Result<Option<Note>, StoreError> {
        let row: Option<NoteRow> = sqlx::query_as("SELECT id, text FROM notes WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Note::from))
    }

    async fn insert(&self, text: &NoteText) -> Result<Note, StoreError> {
        let row: NoteRow =
            sqlx::query_as("INSERT INTO notes (text) VALUES ($1) RETURNING id, text")
                .bind(text.as_str())
                .fetch_one(&self.pool)
                .await?;
        Ok(row.into())
    }

    async fn delete(&self, id: i64) -> Result<bool, StoreError> {
        let result = sqlx::query("DELETE FROM notes WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
