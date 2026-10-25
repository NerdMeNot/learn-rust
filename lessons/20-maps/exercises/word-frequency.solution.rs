use std::collections::HashMap;

/// Count how many times each word appears. Words are separated by whitespace,
/// and "The" and "the" are the same word.
fn word_counts(text: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    assert!(word_counts("").is_empty());

    let counts = word_counts("the Cat saw THE dog");
    assert_eq!(counts.len(), 4);
    assert_eq!(counts["the"], 2);
    assert_eq!(counts["cat"], 1);
    assert_eq!(counts.get("fish"), None);

    let counts = word_counts("École école ÉCOLE");
    assert_eq!(counts.len(), 1);
    assert_eq!(counts["école"], 3);

    println!("all checks passed");
}
