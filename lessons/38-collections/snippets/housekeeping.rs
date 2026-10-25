use std::collections::{BTreeMap, VecDeque};

fn main() {
    // extract_if hands out what it removes; retain just drops what it rejects
    let mut v: Vec<u32> = (1..=10).collect();
    let removed: Vec<u32> = v.extract_if(.., |n| *n % 3 == 0).collect();
    println!("Vec extract_if multiples of 3: removed {removed:?}, left {v:?}");

    let mut m: BTreeMap<u32, &str> = BTreeMap::from([(1, "a"), (2, "b"), (3, "c"), (4, "d")]);
    let odd: Vec<(u32, &str)> = m.extract_if(.., |k, _| k % 2 == 1).collect();
    println!("BTreeMap extract_if odd keys: removed {odd:?}, left {m:?}");

    let mut q: VecDeque<u32> = (1..=6).collect();
    q.retain(|n| n % 2 == 1);
    println!("VecDeque after retain odd: {q:?}");
}
