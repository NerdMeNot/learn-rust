use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<String, u32> = HashMap::new();
    for word in ["red", "blue", "red"] {
        // `entry` borrows the map mutably and takes the key by value
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }
    println!("red: {}", counts["red"]);
    println!("blue: {}", counts["blue"]);
    println!("keys: {}", counts.len());
}
