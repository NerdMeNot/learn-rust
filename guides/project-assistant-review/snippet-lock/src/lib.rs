//! Step 3, with the read switched to tokio::fs, and the lock taken before it.

use std::sync::{Arc, Mutex};

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("cannot read {path}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("a task that reads a file stopped early")]
    Task(#[from] tokio::task::JoinError),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub error: usize,
    pub warn: usize,
    pub info: usize,
    pub slowest_ms: u64,
}

pub async fn summarize(paths: Vec<String>) -> Result<Stats, LogError> {
    let stats = Arc::new(Mutex::new(Stats::default()));

    let mut handles = Vec::new();
    for path in paths {
        let stats = Arc::clone(&stats);
        handles.push(tokio::spawn(async move {
            let mut stats = stats.lock().unwrap();
            let contents = match tokio::fs::read_to_string(&path).await {
                Ok(contents) => contents,
                Err(source) => return Err(LogError::Read { path, source }),
            };
            stats.info += contents.lines().count();
            Ok(())
        }));
    }
    for handle in handles {
        handle.await??;
    }

    let stats = *stats.lock().unwrap();
    Ok(stats)
}
