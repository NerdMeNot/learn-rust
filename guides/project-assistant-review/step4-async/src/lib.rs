//! Counts log lines per level across several files.

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

impl Stats {
    /// Adds another file's numbers to these.
    fn add(&mut self, other: Stats) {
        self.error += other.error;
        self.warn += other.warn;
        self.info += other.info;
        self.slowest_ms = self.slowest_ms.max(other.slowest_ms);
    }
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

/// Reads one file and counts it. Nothing is shared: the task owns its own `Stats`.
async fn count_file(path: String) -> Result<Stats, LogError> {
    let contents = match tokio::fs::read_to_string(&path).await {
        Ok(contents) => contents,
        Err(source) => return Err(LogError::Read { path, source }),
    };
    let mut stats = Stats::default();
    for (i, line) in contents.lines().enumerate() {
        if let Err(value) = count_line(line, &mut stats) {
            let line = i + 1;
            return Err(LogError::BadDuration { path, line, value });
        }
    }
    Ok(stats)
}

/// Reads every file at once and adds up what they contain. The results are awaited in
/// the order of `paths`, so if several files fail, the error is the first one's.
pub async fn summarize(paths: Vec<String>) -> Result<Stats, LogError> {
    let handles: Vec<_> = paths.into_iter().map(|path| tokio::spawn(count_file(path))).collect();

    let mut total = Stats::default();
    for handle in handles {
        total.add(handle.await??);
    }
    Ok(total)
}
