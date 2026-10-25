use std::collections::HashMap;

fn main() {
    let words = ["fig", "apple", "kiwi", "pear", "plum", "lime", "nut"];

    let mut by_length: HashMap<usize, Vec<&str>> = HashMap::new();
    for word in words {
        by_length.entry(word.len()).or_default().push(word);
    }

    let mut lengths: Vec<_> = by_length.keys().collect();
    lengths.sort();
    for length in lengths {
        println!("{length}: {:?}", by_length[length]);
    }
}
