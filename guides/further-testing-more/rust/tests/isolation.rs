//! Each test below builds its own state, so they can run in any order or at the same time.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Hands out ids starting at 1. The counter belongs to the value, not to the process.
struct Ids(AtomicUsize);

impl Ids {
    fn new() -> Ids {
        Ids(AtomicUsize::new(1))
    }
    fn next(&self) -> usize {
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

#[test]
fn creates_alice() {
    let ids = Ids::new();
    assert_eq!(ids.next(), 1);
}

#[test]
fn creates_bob() {
    let ids = Ids::new();
    assert_eq!(ids.next(), 1);
}
