use std::collections::HashMap;

use crate::words::words;

pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in words(text) {
        let lower = word.to_lowercase();
        *counts.entry(lower).or_insert(0) += 1;
        println!("counted {lower}");
    }
    counts
}
