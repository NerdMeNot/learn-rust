use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    counts.insert("cat", 0);
    for w in "cat cat".split(' ') {
        *counts.entry(w).or_insert(0) += 1; // insert 0 if missing, then add 1
    }
    println!("{counts:?}");
}
