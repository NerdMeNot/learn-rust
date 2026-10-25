use std::collections::{BTreeSet, HashSet};

fn main() {
    let mut seen: HashSet<&str> = HashSet::new();
    println!("insert ana: {}", seen.insert("ana"));
    println!("insert ana again: {}", seen.insert("ana"));
    println!("contains bo: {}", seen.contains("bo"));

    let mine = BTreeSet::from([1, 2, 3, 4]);
    let yours = BTreeSet::from([3, 4, 5]);
    let both: Vec<_> = mine.intersection(&yours).collect();
    let either: Vec<_> = mine.union(&yours).collect();
    let only_mine: Vec<_> = mine.difference(&yours).collect();
    println!("both: {both:?}");
    println!("either: {either:?}");
    println!("only mine: {only_mine:?}");
}
