//! The storage layer: one trait, two implementations.

mod memory;
mod sqlite;

pub use memory::MemoryStore;
pub use sqlite::SqliteStore;

use crate::domain::{Note, NoteText};
use std::future::Future;

/// Something went wrong in the backend. The domain does not care what: a
/// database error, a full disk and a dropped connection are all "storage failed".
#[derive(Debug, thiserror::Error)]
#[error("storage backend failed")]
pub struct StoreError {
    #[source]
    source: Box<dyn std::error::Error + Send + Sync>,
}

impl StoreError {
    pub fn new(source: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        StoreError {
            source: source.into(),
        }
    }
}

/// The boundary between the HTTP layer and wherever notes live.
///
/// The methods return `impl Future + Send` instead of using `async fn` so the
/// trait can promise `Send`, which axum needs. An implementation may still
/// write `async fn`.
pub trait NoteStore: Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = Result<Vec<Note>, StoreError>> + Send;
    fn get(&self, id: i64) -> impl Future<Output = Result<Option<Note>, StoreError>> + Send;
    fn insert(&self, text: &NoteText) -> impl Future<Output = Result<Note, StoreError>> + Send;
    /// `true` if a note was deleted, `false` if there was none with that id.
    fn delete(&self, id: i64) -> impl Future<Output = Result<bool, StoreError>> + Send;
}
