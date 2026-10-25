use std::collections::HashMap;

use crate::words::words;

pub fn count_words(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in words(text) {
        let lower = word.to_lowercase();
        *counts.entry(lower.as_str()).or_insert(0) += 1;
    }
    counts
}
