use std::{io, path::PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FileStats {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

pub fn count(text: &str) -> FileStats {
    FileStats {
        lines: text.lines().count(),
        words: text.split_whitespace().count(),
        bytes: text.len(),
    }
}

/// Reads one file and counts it. `tokio::fs` runs the read on a thread meant for blocking
/// work, so the async runtime's own threads stay free.
pub async fn index_file(path: PathBuf) -> io::Result<FileStats> {
    let text = tokio::fs::read_to_string(path).await?;
    Ok(count(&text))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_lines_words_and_bytes() {
        let stats = count("one two\nthree\n");
        assert_eq!(stats, FileStats { lines: 2, words: 3, bytes: 14 });
    }

    #[test]
    fn empty_text_is_all_zero() {
        assert_eq!(count(""), FileStats::default());
    }

    #[test]
    fn a_last_line_without_a_newline_still_counts() {
        assert_eq!(count("a b").lines, 1);
    }

    #[tokio::test]
    async fn a_missing_file_is_a_not_found_error() {
        let err = index_file("no/such/file".into()).await.unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::NotFound);
    }
}
