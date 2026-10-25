use std::collections::HashMap;

// `entry(&mut self, key: K)` takes the key by value, and `K` is `String` here.
fn count_words(words: &[&str]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in words {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

fn main() {
    let counts = count_words(&["red", "blue", "red"]);
    assert_eq!(counts["red"], 2);
    assert_eq!(counts["blue"], 1);
    assert_eq!(counts.get("green"), None);
    assert_eq!(counts.len(), 2);
    assert!(count_words(&[]).is_empty());
    println!("all checks passed");
}
