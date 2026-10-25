use std::collections::{BTreeSet, HashSet};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Tag(String);

fn main() {
    let mut hashed = HashSet::new(); // Tag has Hash and Eq: fine
    hashed.insert(Tag("a".into()));

    let mut sorted = BTreeSet::new(); // Tag has no Ord
    sorted.insert(Tag("a".into()));
    println!("{} {}", hashed.len(), sorted.len());
}
