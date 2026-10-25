use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

/// A cached number. It counts its clones, because a cache that copies entries (or the
/// whole map) to look one up has lost the point. Read the number out with `.0` instead.
pub struct Entry(u64);

static ENTRY_CLONES: AtomicUsize = AtomicUsize::new(0);

impl Clone for Entry {
    fn clone(&self) -> Entry {
        ENTRY_CLONES.fetch_add(1, Ordering::SeqCst);
        Entry(self.0)
    }
}

pub type Table = Mutex<HashMap<u32, Entry>>;

/// The cached value for `key`. On a miss, call `compute` and store its result.
/// `compute` may be slow, so no lock may be held while it runs. If another thread
/// stored a value for `key` while `compute` ran, keep that one and return it.
/// This is not "single-flight": two threads that miss together may both compute.
pub fn get_or_compute(table: &Table, key: u32, compute: impl FnOnce() -> u64) -> u64 {
    todo!("look first; compute with the lock released; store with entry().or_insert")
}

/// Put a new table in an `Arc`. Start `threads` threads, each calling
/// `get_or_compute(&table, k, || compute(k))` for every key in `0..keys`. Join them
/// all, then return the table.
pub fn warm(keys: u32, threads: usize, compute: fn(u32) -> u64) -> Arc<Table> {
    todo!(
        "Arc::new, then Arc::clone for each thread, move it in, and join (add `use std::thread;`)"
    )
}

static CALLS: AtomicUsize = AtomicUsize::new(0);

fn square(k: u32) -> u64 {
    CALLS.fetch_add(1, Ordering::SeqCst);
    u64::from(k) * u64::from(k)
}

fn main() {
    // A miss computes once; a hit does not compute.
    let table: Table = Mutex::new(HashMap::new());
    let mut calls = 0;
    assert_eq!(
        get_or_compute(&table, 3, || {
            calls += 1;
            9
        }),
        9
    );
    assert_eq!(
        get_or_compute(&table, 3, || panic!("a hit must not compute")),
        9
    );
    assert_eq!(calls, 1, "computed once");

    // The lock is free while `compute` runs.
    let table: Table = Mutex::new(HashMap::new());
    let value = get_or_compute(&table, 4, || {
        assert!(
            table.try_lock().is_ok(),
            "compute ran while the lock was held"
        );
        16
    });
    assert_eq!(value, 16);

    // Someone else stores key 7 while we compute: theirs wins, and we return theirs.
    let table: Table = Mutex::new(HashMap::new());
    let value = get_or_compute(&table, 7, || {
        table.lock().unwrap().insert(7, Entry(999));
        49
    });
    assert_eq!(value, 999, "keep the value that was stored first");
    assert_eq!(get_or_compute(&table, 7, || panic!("already stored")), 999);

    // `warm` shares one table between threads, and every thread has finished when it returns.
    let table = warm(50, 4, square);
    assert_eq!(
        Arc::strong_count(&table),
        1,
        "join every thread before returning"
    );
    for k in 0..50 {
        let hit = get_or_compute(&table, k, || panic!("warm did not store key {k}"));
        assert_eq!(hit, u64::from(k) * u64::from(k));
    }
    let calls = CALLS.load(Ordering::SeqCst);
    assert!(
        (50..=200).contains(&calls),
        "expected between 50 and 200 computations, got {calls}"
    );

    assert_eq!(
        ENTRY_CLONES.load(Ordering::SeqCst),
        0,
        "an Entry was cloned: read the number out with `.0` instead of copying"
    );
    println!("all checks passed");
}
