use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::atomic::{AtomicUsize, Ordering};

static LIVE: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

/// Test scaffolding: a reading that counts how many readings exist at once. A real
/// stream is too big to hold, so `top_k` must never keep more than a few of these alive.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Reading(u32);

impl Reading {
    fn new(value: u32) -> Reading {
        let live = LIVE.fetch_add(1, Ordering::SeqCst) + 1;
        PEAK.fetch_max(live, Ordering::SeqCst);
        Reading(value)
    }
}

impl Drop for Reading {
    fn drop(&mut self) {
        LIVE.fetch_sub(1, Ordering::SeqCst);
    }
}

/// The `k` largest items in the stream, largest first. Equal items each count. A stream
/// with fewer than `k` items gives all of them. Use a `BinaryHeap`: sorting the whole
/// stream would keep every reading alive.
pub fn top_k<T: Ord>(stream: impl Iterator<Item = T>, k: usize) -> Vec<T> {
    todo!("a min-heap (Reverse) holding the k best so far; pop when it grows past k")
}

fn run(values: &[u32], k: usize) -> (Vec<u32>, usize) {
    LIVE.store(0, Ordering::SeqCst);
    PEAK.store(0, Ordering::SeqCst);
    let top: Vec<u32> = top_k(values.iter().map(|&v| Reading::new(v)), k)
        .into_iter()
        .map(|r| r.0)
        .collect();
    assert_eq!(
        LIVE.load(Ordering::SeqCst),
        0,
        "every reading must be dropped"
    );
    (top, PEAK.load(Ordering::SeqCst))
}

fn main() {
    let (top, _) = run(&[4, 9, 1, 7, 9, 3], 3);
    assert_eq!(top, [9, 9, 7], "equal values each count");

    let (top, _) = run(&[5, 2], 5);
    assert_eq!(top, [5, 2], "fewer readings than k gives them all");

    let (top, _) = run(&[], 3);
    assert!(top.is_empty(), "an empty stream");

    let (top, _) = run(&[8, 6], 0);
    assert!(top.is_empty(), "k = 0");

    // A long stream: 5,000 readings, keep 5.
    let values: Vec<u32> = (0..5_000u32).map(|i| (i * 7_919 + 13) % 10_007).collect();
    let mut expected = values.clone();
    expected.sort_by(|a, b| b.cmp(a));
    expected.truncate(5);
    let (top, peak) = run(&values, 5);
    assert_eq!(top, expected);
    assert!(
        peak <= 6,
        "up to {peak} readings were alive at once: keep only k (plus the new one)"
    );
    println!("all checks passed");
}
