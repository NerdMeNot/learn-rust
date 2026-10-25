use std::collections::{BTreeSet, HashMap, HashSet};

fn main() {
    let words = ["apple", "bob", "apple", "cat"];

    // The type you ask for decides what collect builds.
    let v: Vec<&str> = words.iter().copied().collect();
    println!("Vec: {v:?}");

    let s: String = words.iter().map(|w| w.chars().next().unwrap()).collect();
    println!("String from chars: {s}");

    let joined: String = words.iter().copied().collect();
    println!("String from &strs: {joined}");

    // A set drops duplicates. HashSet order is random, so we only print the length.
    let unique: HashSet<&str> = words.iter().copied().collect();
    println!("HashSet len: {}", unique.len());
    let sorted: BTreeSet<&str> = words.iter().copied().collect();
    println!("BTreeSet: {sorted:?}");

    // Pairs collect into a map. A repeated key keeps the last value.
    let lengths: HashMap<&str, usize> = words.iter().map(|w| (*w, w.len())).collect();
    println!("HashMap[\"apple\"]: {}", lengths["apple"]);

    // The turbofish ::<> says the same thing without a `let` type.
    let n = (1..=3).collect::<Vec<i32>>();
    // `_` lets Rust work out the item type: you only name the container.
    let m: Vec<_> = (1..=3).map(|x| x * 10).collect();
    println!("turbofish: {n:?}, with _: {m:?}");
}
