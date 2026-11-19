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
///
/// A line is `<time> <LEVEL> <message...>`. Only the second field is the level, and
/// only a field that starts with `ms=` is the duration.
fn count_line(line: &str, stats: &mut Stats) -> Result<(), String> {
    let mut fields = line.split_whitespace();
    match fields.nth(1) {
        Some("ERROR") => stats.error += 1,
        Some("WARN") => stats.warn += 1,
        Some("INFO") => stats.info += 1,
        _ => return Ok(()),
    }

    if let Some(value) = fields.find_map(|field| field.strip_prefix("ms=")) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn count(lines: &[&str]) -> Result<Stats, String> {
        let mut stats = Stats::default();
        for line in lines {
            count_line(line, &mut stats)?;
        }
        Ok(stats)
    }

    #[test]
    fn counts_each_level_and_keeps_the_slowest() -> Result<(), String> {
        let stats = count(&[
            "2026-11-03T10:00:01Z INFO GET / status=200 ms=5",
            "2026-11-03T10:00:02Z WARN GET /login status=401 ms=70",
            "2026-11-03T10:00:03Z ERROR POST /orders status=500 ms=9",
        ])?;
        let expected = Stats { error: 1, warn: 1, info: 1, slowest_ms: 70 };
        assert_eq!(stats, expected);
        Ok(())
    }

    #[test]
    fn blank_and_unknown_lines_are_skipped() -> Result<(), String> {
        let stats = count(&["", "   ", "2026-11-03T10:00:01Z DEBUG cache warm ms=900"])?;
        assert_eq!(stats, Stats::default());
        Ok(())
    }

    #[test]
    fn a_bad_duration_is_an_error_with_its_text() {
        let result = count(&["2026-11-03T10:00:01Z INFO GET / ms=fast"]);
        assert_eq!(result, Err("fast".to_owned()));
    }

    // The two cases the draft got wrong. They are also in snippet-edge/edge_cases.rs,
    // which the page runs against step 4's code.
    #[test]
    fn a_level_word_inside_the_message_is_not_the_level() -> Result<(), String> {
        let stats = count(&["2026-11-03T10:00:07Z WARN payment gateway said ERROR, retrying"])?;
        assert_eq!(stats, Stats { warn: 1, ..Stats::default() });
        Ok(())
    }

    #[test]
    fn a_field_that_ends_in_ms_is_not_the_duration() -> Result<(), String> {
        let stats = count(&["2026-11-03T10:00:08Z INFO batch done items=40 ms=12"])?;
        assert_eq!(stats.slowest_ms, 12);
        Ok(())
    }
}
