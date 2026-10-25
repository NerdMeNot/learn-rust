use std::collections::HashMap;

fn main() {
    let votes = ["ana", "bo", "ana", "cy", "ana", "bo"];

    let mut tally: HashMap<&str, u32> = HashMap::new();
    for name in votes {
        *tally.entry(name).or_insert(0) += 1;
    }

    // `iter()` yields the pairs one at a time. `collect()` gathers them into a Vec,
    // and a Vec of pairs can be sorted.
    let mut results: Vec<_> = tally.iter().collect();
    results.sort();
    for (name, count) in results {
        println!("{name}: {count}");
    }
}
