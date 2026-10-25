use std::collections::HashSet;
use std::hint::black_box;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Mutex};
use std::thread::{self, ThreadId};
use std::time::Duration;

/// Runs `f` on every item, each item on its own thread, and returns the results in the
/// same order as `items`.
fn parallel_map<F>(items: Vec<u64>, f: F) -> Vec<u64>
where
    F: Fn(u64) -> u64 + Send + Sync + 'static,
{
    let count = items.len();
    let f = Arc::new(f);
    let (tx, rx) = mpsc::channel();
    for (index, item) in items.into_iter().enumerate() {
        let (tx, f) = (tx.clone(), Arc::clone(&f));
        thread::spawn(move || tx.send((index, f(item))).unwrap());
    }
    drop(tx); // every thread holds a clone; this is the last one that is not working

    let mut results = vec![0; count];
    for (index, result) in rx {
        results[index] = result;
    }
    results
}

// Gives up when `f` takes too long: a receiving loop that never ends is the likeliest bug.
fn within<T: Send + 'static>(what: &str, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(Duration::from_secs(20)) {
        Ok(value) => value,
        Err(RecvTimeoutError::Timeout) => {
            panic!("{what}: still waiting after 20 s. Is a Sender still alive somewhere?")
        }
        Err(RecvTimeoutError::Disconnected) => panic!("{what}: the function panicked"),
    }
}

fn main() {
    let squares = within("squares", || parallel_map(vec![1, 2, 3, 4], |x| x * x));
    assert_eq!(squares, [1, 4, 9, 16]);

    // results that get smaller as the input grows: sorting the values is not enough
    let falling = within("falling", || parallel_map(vec![1, 2, 3, 4], |x| 10 - x));
    assert_eq!(
        falling,
        [9, 8, 7, 6],
        "results must follow the input order, not their own size"
    );

    // no items: the receiving loop must still end
    let none = within("no items", || parallel_map(vec![], |x| x + 1));
    assert!(none.is_empty());

    // Item 0 does far more work than the rest, so it finishes last. Results must still
    // come back in input order, run after run.
    for round in 0..20 {
        let items: Vec<u64> = (0..200).collect();
        let out = within("200 items", move || {
            parallel_map(items, |x| {
                if x == 0 {
                    let mut acc = 0u64;
                    // black_box stops the optimizer from deleting this busy loop
                    for i in 0..3_000_000u64 {
                        acc = black_box(acc.wrapping_add(i));
                    }
                    black_box(acc);
                }
                (x * 7919) % 1000 // neither increasing nor decreasing
            })
        });
        let expected: Vec<u64> = (0..200).map(|x| (x * 7919) % 1000).collect();
        assert_eq!(out, expected, "round {round}: results are out of order");
    }

    // the work must not run on the calling thread
    let main_id = thread::current().id();
    let seen: Arc<Mutex<HashSet<ThreadId>>> = Arc::new(Mutex::new(HashSet::new()));
    let seen2 = Arc::clone(&seen);
    within("threads", move || {
        parallel_map(vec![1, 2, 3, 4, 5, 6], move |x| {
            seen2.lock().unwrap().insert(thread::current().id());
            x
        })
    });
    let seen = seen.lock().unwrap();
    assert!(!seen.contains(&main_id), "f ran on the main thread");
    assert_eq!(seen.len(), 6, "each item should get its own thread");

    println!("all checks passed");
}
