use std::collections::HashMap;

use crate::words::words;

/// Counts how often each word appears, ignoring case.
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words(text) {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

/// The `n` most common words, most common first.
pub fn top_n(counts: HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
    let mut entries: Vec<(String, usize)> = counts.into_iter().collect();
    entries.sort_by(|a, b| b.1.cmp(&a.1));
    entries.truncate(n);
    entries
}
