use super::{NoteStore, StoreError};
use crate::domain::{Note, NoteText};
use std::sync::Mutex;

/// Notes in a `Vec`. Used by tests, and a second implementation that proves
/// the trait is not shaped around SQL.
#[derive(Default)]
pub struct MemoryStore {
    inner: Mutex<Inner>,
}

#[derive(Default)]
struct Inner {
    last_id: i64,
    notes: Vec<Note>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl NoteStore for MemoryStore {
    async fn list(&self) -> Result<Vec<Note>, StoreError> {
        Ok(self.inner.lock().expect("lock poisoned").notes.clone())
    }

    async fn get(&self, id: i64) -> Result<Option<Note>, StoreError> {
        let inner = self.inner.lock().expect("lock poisoned");
        Ok(inner.notes.iter().find(|n| n.id == id).cloned())
    }

    async fn insert(&self, text: &NoteText) -> Result<Note, StoreError> {
        let mut inner = self.inner.lock().expect("lock poisoned");
        inner.last_id += 1; // ids are never reused, like AUTOINCREMENT in the SQL table
        let note = Note {
            id: inner.last_id,
            text: text.as_str().to_string(),
        };
        inner.notes.push(note.clone());
        Ok(note)
    }

    async fn delete(&self, id: i64) -> Result<bool, StoreError> {
        let mut inner = self.inner.lock().expect("lock poisoned");
        let before = inner.notes.len();
        inner.notes.retain(|n| n.id != id);
        Ok(inner.notes.len() < before)
    }
}
