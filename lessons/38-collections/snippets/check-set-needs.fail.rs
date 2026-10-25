use std::collections::{BTreeSet, HashSet};

#[derive(Hash, PartialEq, Eq)]
struct Tag(u8); // no Clone

fn main() {
    let hashed = HashSet::from([1, 2]);
    let sorted = BTreeSet::from([2, 3]);
    let mixed: Vec<_> = hashed.intersection(&sorted).collect();

    let a = HashSet::from([Tag(1)]);
    let b = HashSet::from([Tag(1)]);
    let both = &a & &b;
    println!("{} {}", mixed.len(), both.len());
}
