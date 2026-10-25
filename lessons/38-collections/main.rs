use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    let text = "the cat saw the dog and the cat ran";

    // HashMap: count words. Iteration order is unspecified, so sort before printing.
    let mut counts: HashMap<&str, u32> = HashMap::new();
    for w in text.split_whitespace() {
        *counts.entry(w).or_insert(0) += 1;
    }
    let mut pairs: Vec<(&str, u32)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!("top: {:?}", &pairs[..3]);

    // BTreeMap: keys always come out sorted
    let mut by_name = BTreeMap::new();
    by_name.insert("zoe", 31);
    by_name.insert("ana", 30);
    by_name.insert("mia", 28);
    println!("sorted: {by_name:?}");

    // HashSet: unique items, fast "is it in there?"
    let seen: HashSet<&str> = text.split_whitespace().collect();
    println!(
        "{} unique words, has dog: {}",
        seen.len(),
        seen.contains("dog")
    );

    // VecDeque: a queue. Push at the back, pop at the front, both cheap.
    let mut queue = VecDeque::from(["a", "b"]);
    queue.push_back("c");
    let first = queue.pop_front();
    println!("served {first:?}, waiting {queue:?}");

    // BinaryHeap pops the largest first. Wrap in Reverse to get the smallest.
    let mut heap = BinaryHeap::new();
    for n in [5, 1, 8, 3] {
        heap.push(Reverse(n));
    }
    print!("smallest first:");
    while let Some(Reverse(n)) = heap.pop() {
        print!(" {n}");
    }
    println!();
}
