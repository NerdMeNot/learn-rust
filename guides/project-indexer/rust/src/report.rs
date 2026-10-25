use std::{collections::BTreeMap, io, path::PathBuf};

use crate::index::FileStats;

/// What the indexer found: a `BTreeMap` keeps its keys sorted, so the order never depends
/// on which file finished first.
pub type Results = BTreeMap<PathBuf, io::Result<FileStats>>;

/// One line per file in path order, then a total. Also returns how many files failed.
pub fn render(results: &Results) -> (String, usize) {
    let mut out = format!("{:<30} {:>5} {:>6} {:>7}\n", "file", "lines", "words", "bytes");
    let (mut total, mut files, mut failed) = (FileStats::default(), 0, 0);
    for (path, result) in results {
        let name = path.to_string_lossy();
        match result {
            Ok(s) => {
                out += &format!("{name:<30} {:>5} {:>6} {:>7}\n", s.lines, s.words, s.bytes);
                total.lines += s.lines;
                total.words += s.words;
                total.bytes += s.bytes;
                files += 1;
            }
            Err(e) => {
                out += &format!("{name:<30} error: {e}\n");
                failed += 1;
            }
        }
    }
    out += &format!("{files} files, {} lines, {} words, {} bytes", total.lines, total.words, total.bytes);
    if failed > 0 {
        out += &format!(", {failed} failed");
    }
    out.push('\n');
    (out, failed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_come_out_in_path_order_and_failures_are_counted() {
        let mut results = Results::new();
        results.insert("b.txt".into(), Ok(FileStats { lines: 1, words: 2, bytes: 6 }));
        results.insert("a.txt".into(), Err(io::Error::new(io::ErrorKind::NotFound, "gone")));
        let (text, failed) = render(&results);
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(failed, 1);
        assert!(lines[1].starts_with("a.txt"), "{text}");
        assert!(lines[2].starts_with("b.txt"), "{text}");
        assert_eq!(lines[3], "1 files, 1 lines, 2 words, 6 bytes, 1 failed");
    }
}
