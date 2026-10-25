use std::collections::{BTreeSet, BinaryHeap, HashSet};

fn main() {
    let hashed = HashSet::from([1, 2]);
    let sorted = BTreeSet::from([1, 2]);
    let heap = BinaryHeap::from([1, 2]);
    println!("{} {} {}", hashed[0], sorted[0], heap[0]);
}
