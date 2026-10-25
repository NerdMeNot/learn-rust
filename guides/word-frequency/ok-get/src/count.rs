use std::collections::HashMap;

use crate::words::words;

pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words(text) {
        let lower = word.to_lowercase();
        let before = counts.get(&lower).copied().unwrap_or(0);
        counts.insert(lower, before + 1);
    }
    counts
}
