use std::collections::HashMap;

fn main() {
    // Four maps with exactly the same pairs, built one after another in one run.
    for n in 1..=4 {
        let map = HashMap::from([
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
        ]);
        let keys: Vec<_> = map.keys().collect();
        let again: Vec<_> = map.keys().collect();
        println!(
            "map {n}: {keys:?}, same map looped again gives the same order: {}",
            keys == again
        );
    }
}
