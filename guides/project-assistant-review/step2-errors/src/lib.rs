//! Counts log lines per level across several files.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, thiserror::Error)]
pub enum LogError {
    #[error("cannot read {path}")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{path}, line {line}: duration {value:?} is not a whole number of milliseconds")]
    BadDuration {
        path: String,
        line: usize,
        value: String,
    },
    #[error("a task that reads a file stopped early")]
    Task(#[from] tokio::task::JoinError),
}

#[derive(Debug, Default, Clone)]
pub struct Stats {
    pub counts: HashMap<String, usize>,
    pub slowest_ms: u64,
}

/// Counts one line into `stats`. If the duration is not a number, returns its text.
fn count_line(line: &String, stats: &mut Stats) -> Result<(), String> {
    let level = if line.contains("ERROR") {
        "ERROR"
    } else if line.contains("WARN") {
        "WARN"
    } else if line.contains("INFO") {
        "INFO"
    } else {
        return Ok(());
    };
    *stats.counts.entry(level.to_string()).or_insert(0) += 1;

    if let Some(pos) = line.find("ms=") {
        let value = line[pos + 3..].split_whitespace().next().unwrap_or("");
        let ms: u64 = value.parse().map_err(|_| value.to_string())?;
        stats.slowest_ms = stats.slowest_ms.max(ms);
    }
    Ok(())
}

/// Reads every file at once and adds up what they contain.
pub async fn summarize(paths: &Vec<String>) -> Result<Stats, LogError> {
    let stats = Arc::new(Mutex::new(Stats::default()));

    let mut handles = Vec::new();
    for path in paths.clone() {
        let stats = stats.clone();
        handles.push(tokio::spawn(async move {
            let contents = std::fs::read_to_string(&path).map_err(|source| LogError::Read {
                path: path.clone(),
                source,
            })?;
            for (i, line) in contents.lines().enumerate() {
                let mut stats = stats.lock().unwrap();
                count_line(&line.to_string(), &mut stats).map_err(|value| {
                    LogError::BadDuration {
                        path: path.clone(),
                        line: i + 1,
                        value,
                    }
                })?;
            }
            Ok::<(), LogError>(())
        }));
    }
    for handle in handles {
        handle.await??;
    }

    let stats = stats.lock().unwrap().clone();
    Ok(stats)
}
