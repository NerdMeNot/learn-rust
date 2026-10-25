use std::collections::HashMap;

use crate::words::words;

pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words(text) {
        let lower = word.to_lowercase();
        let before = counts.get(&lower);
        counts.insert(lower, before.copied().unwrap_or(0) + 1);
        println!("{word}: was {before:?}");
    }
    counts
}
