use std::collections::HashMap;

/// Count how often each word appears, ignoring case.
fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    counts
}

fn main() {
    let counts = word_count("the cat and The hat");
    let mut words: Vec<_> = counts.iter().collect();
    words.sort();
    for (word, n) in words {
        println!("{word}: {n}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_ignoring_case() {
        let counts = word_count("the cat and The hat");
        assert_eq!(counts["the"], 2);
        assert_eq!(counts["cat"], 1);
    }
}
