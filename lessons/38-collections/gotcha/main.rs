use std::collections::HashMap;

fn main() {
    let mut counts: HashMap<&str, u32> = HashMap::new();
    counts.insert("cat", 0);
    for w in "cat cat".split(' ') {
        counts[w] += 1;
    }
    println!("{counts:?}");
}
