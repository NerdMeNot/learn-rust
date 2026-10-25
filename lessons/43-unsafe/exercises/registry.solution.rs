#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::sync::Mutex;
use std::thread;

// Declare the global state here. It must be a `static` that several threads can use.
// The file forbids `unsafe`, so `static mut` is not an option.
static NAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Records `name` and returns how many names are recorded, counting this one.
fn register(name: &str) -> usize {
    let mut names = NAMES.lock().unwrap();
    names.push(name.to_string());
    names.len()
}

/// Every recorded name, sorted.
fn names() -> Vec<String> {
    let mut all = NAMES.lock().unwrap().clone();
    all.sort();
    all
}

fn main() {
    let counts: Vec<usize> = thread::scope(|s| {
        let handles: Vec<_> = (0..4)
            .map(|t| {
                s.spawn(move || {
                    (0..250)
                        .map(|i| register(&format!("t{t}-{i:03}")))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });

    // every call saw a different count, 1 through 1000: no update was lost
    let unique: HashSet<usize> = counts.iter().copied().collect();
    assert_eq!(unique.len(), 1000, "two calls returned the same count");
    assert_eq!(counts.iter().max(), Some(&1000));

    let all = names();
    assert_eq!(all.len(), 1000);
    assert!(
        all.windows(2).all(|w| w[0] < w[1]),
        "names must be sorted and unique"
    );
    assert_eq!(all[0], "t0-000");
    assert_eq!(all[999], "t3-249");

    // a late registration must show up too: the global holds the names, not just a count
    assert_eq!(register("zzz"), 1001);
    assert!(
        names().contains(&String::from("zzz")),
        "names() lost a name"
    );
    println!("all checks passed");
}
