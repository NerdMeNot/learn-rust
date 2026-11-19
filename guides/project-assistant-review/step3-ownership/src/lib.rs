//! Counts log lines per level across several files.

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

/// Three counters and a maximum: all numbers, so the whole struct is `Copy`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stats {
    pub error: usize,
    pub warn: usize,
    pub info: usize,
    pub slowest_ms: u64,
}

/// Counts one line into `stats`. If the duration is not a number, returns its text.
fn count_line(line: &str, stats: &mut Stats) -> Result<(), String> {
    if line.contains("ERROR") {
        stats.error += 1;
    } else if line.contains("WARN") {
        stats.warn += 1;
    } else if line.contains("INFO") {
        stats.info += 1;
    } else {
        return Ok(());
    }

    if let Some(pos) = line.find("ms=") {
        let value = line[pos + 3..].split_whitespace().next().unwrap_or("");
        let ms: u64 = value.parse().map_err(|_| value.to_owned())?;
        stats.slowest_ms = stats.slowest_ms.max(ms);
    }
    Ok(())
}

/// Reads every file at once and adds up what they contain.
pub async fn summarize(paths: Vec<String>) -> Result<Stats, LogError> {
    let stats = Arc::new(Mutex::new(Stats::default()));

    let mut handles = Vec::new();
    for path in paths {
        let stats = Arc::clone(&stats);
        handles.push(tokio::spawn(async move {
            let contents = match std::fs::read_to_string(&path) {
                Ok(contents) => contents,
                Err(source) => return Err(LogError::Read { path, source }),
            };
            for (i, line) in contents.lines().enumerate() {
                let mut stats = stats.lock().unwrap();
                if let Err(value) = count_line(line, &mut stats) {
                    let line = i + 1;
                    return Err(LogError::BadDuration { path, line, value });
                }
            }
            Ok(())
        }));
    }
    for handle in handles {
        handle.await??;
    }

    let stats = *stats.lock().unwrap();
    Ok(stats)
}
