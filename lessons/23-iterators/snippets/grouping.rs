use std::collections::{BTreeMap, HashMap};

fn main() {
    let text = "the cat and the hat and the bat";

    // Counting: entry finds the key or inserts 0, and gives you the value to change.
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    // HashMap order is random. Collect and sort to print it in a fixed order.
    let mut by_count: Vec<(&str, usize)> = counts.into_iter().collect();
    by_count.sort_by_key(|&(word, n)| (std::cmp::Reverse(n), word));
    println!("counts: {by_count:?}");

    // Grouping: or_default() starts each group with an empty Vec.
    // A BTreeMap keeps the keys sorted.
    let mut by_len: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for word in ["fig", "kiwi", "pear", "yam", "plum"] {
        by_len.entry(word.len()).or_default().push(word);
    }
    println!("grouped by length: {by_len:?}");

    // Runs of neighbours that belong together: chunk_by on a slice.
    // Only neighbours are grouped, like Unix `uniq`.
    let readings = [1, 2, 3, 7, 8, 12];
    let runs: Vec<&[i32]> = readings.chunk_by(|a, b| b - a == 1).collect();
    println!("chunk_by consecutive: {runs:?}");
}
