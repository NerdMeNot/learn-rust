//! Helpers shared by the files in tests/. Each file in tests/ is its own crate, so a file
//! that wants these says `mod common;`. The folder form (common/mod.rs) keeps Cargo from
//! treating this file as a test crate of its own.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{env, fs, process};

/// A fresh folder in the system temp dir that is deleted when the value is dropped.
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub fn new(label: &str) -> TempDir {
        // The process id keeps two test runs apart, the counter keeps two tests apart.
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, Ordering::SeqCst);
        let path = env::temp_dir().join(format!("timekeep-{label}-{}-{n}", process::id()));
        fs::create_dir_all(&path).expect("create the temp dir");
        TempDir { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// A fixture: a folder that already holds two notes.
pub fn dir_with_two_notes() -> TempDir {
    let dir = TempDir::new("two-notes");
    timekeep::store::save_note(dir.path(), "standup", "9:30").unwrap();
    timekeep::store::save_note(dir.path(), "review", "14:00").unwrap();
    dir
}
