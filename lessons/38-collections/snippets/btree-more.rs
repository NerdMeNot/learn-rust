use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

fn main() {
    // entry works like HashMap's, and the keys stay sorted.
    let mut by_len: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for w in ["fig", "apple", "kiwi", "pear", "plum", "date", "lime"] {
        by_len.entry(w.len()).or_default().push(w);
    }
    println!("by length: {by_len:?}");

    let mut log: BTreeMap<u32, &str> = BTreeMap::new();
    for (t, e) in [
        (10, "boot"),
        (25, "login"),
        (40, "upload"),
        (55, "logout"),
        (70, "halt"),
    ] {
        log.insert(t, e);
    }
    // range takes any range of keys, or a pair of Bounds for the odd cases.
    println!("25..=55: {:?}", log.range(25..=55).collect::<Vec<_>>());
    println!(
        "after 25, not 25: {:?}",
        log.range((Excluded(25), Unbounded))
            .map(|(t, _)| *t)
            .collect::<Vec<_>>()
    );
    println!(
        "up to and including 40: {:?}",
        log.range((Unbounded, Included(40)))
            .map(|(t, _)| *t)
            .collect::<Vec<_>>()
    );
    // the nearest key at or before 30, and at or after 30
    println!("floor(30): {:?}", log.range(..=30).next_back());
    println!("ceiling(30): {:?}", log.range(30..).next());

    // smallest and largest: peek or remove
    println!(
        "first: {:?}, last: {:?}",
        log.first_key_value(),
        log.last_key_value()
    );
    println!("pop_first: {:?}", log.pop_first());
    println!("pop_last: {:?}", log.pop_last());

    // split_off moves everything from a key upward into a new map
    let later = log.split_off(&40);
    println!("kept: {log:?}, split off: {later:?}");
}
