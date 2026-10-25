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
