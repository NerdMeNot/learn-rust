use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

mod slot {
    use super::*;

    thread_local! {
        static FAILED_WITH: Cell<Option<usize>> = const { Cell::new(None) };
    }

    /// A number that threads share. The atomic inside is private to this module, so the
    /// only ways in are the three methods below.
    pub struct Slot(AtomicUsize);

    impl Slot {
        pub fn new(value: usize) -> Slot {
            Slot(AtomicUsize::new(value))
        }

        pub fn load(&self) -> usize {
            FAILED_WITH.with(|failed| failed.set(None));
            self.0.load(Ordering::SeqCst)
        }

        /// A plain store. Provided so the checks can play the part of a rival thread.
        pub fn store(&self, value: usize) {
            self.0.store(value, Ordering::SeqCst);
        }

        /// Store `new` only if the slot still holds `current`. Returns `Ok(current)`, or
        /// `Err(value)` with the value that was there instead. Asking again with the
        /// `current` that just failed, without a `load` in between, would fail forever,
        /// so that panics.
        pub fn compare_exchange(&self, current: usize, new: usize) -> Result<usize, usize> {
            FAILED_WITH.with(|failed| {
                assert_ne!(
                    failed.get(),
                    Some(current),
                    "retry with the value compare_exchange gave back, not the one that just failed"
                );
                let result =
                    self.0
                        .compare_exchange(current, new, Ordering::SeqCst, Ordering::SeqCst);
                failed.set(result.is_err().then_some(current));
                result
            })
        }
    }
}

use slot::Slot;

/// Update the slot with `f`, without a lock. Read the value, ask `f` for a replacement,
/// and store it with `compare_exchange`. If another thread changed the slot in between,
/// do not store: ask `f` again with the value that is there now.
/// - `f` returns `None`: stop and return `Err(value it was given)`.
/// - A swap succeeds: return `Ok(value it replaced)`.
pub fn update(slot: &Slot, mut f: impl FnMut(usize) -> Option<usize>) -> Result<usize, usize> {
    let mut current = slot.load();
    loop {
        let Some(new) = f(current) else {
            return Err(current);
        };
        match slot.compare_exchange(current, new) {
            Ok(previous) => return Ok(previous),
            Err(found) => current = found,
        }
    }
}

use std::sync::Arc;
use std::thread;

/// Start `threads` threads that share one slot (starting at 0) and each call `update`
/// to add 1 while the slot is below `limit`, until `update` says no. Join them all and
/// return how many increments each thread made.
pub fn claim_all(limit: usize, threads: usize) -> Vec<usize> {
    let slot = Arc::new(Slot::new(0));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let slot = Arc::clone(&slot);
            thread::spawn(move || {
                let mut mine = 0;
                while update(&slot, |n| (n < limit).then_some(n + 1)).is_ok() {
                    mine += 1;
                }
                mine
            })
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    // A closure that says yes: the old value comes back and the slot changes.
    let slot = Slot::new(5);
    assert_eq!(update(&slot, |n| Some(n + 1)), Ok(5));
    assert_eq!(slot.load(), 6);

    // A closure that says no: the current value comes back and nothing changes.
    assert_eq!(update(&slot, |_| None), Err(6));
    assert_eq!(slot.load(), 6);

    // A rival changes the slot while `f` is thinking. The first swap must fail, and
    // `f` must be asked again with the rival's value.
    let slot = Slot::new(5);
    let mut seen = Vec::new();
    let result = update(&slot, |n| {
        seen.push(n);
        if seen.len() == 1 {
            slot.store(100);
        }
        Some(n + 1)
    });
    assert_eq!(
        seen,
        [5, 100],
        "ask f again with the value that is there now"
    );
    assert_eq!(result, Ok(100));
    assert_eq!(
        slot.load(),
        101,
        "the rival's value must not be overwritten"
    );

    // The rival's value can also make f say no.
    let slot = Slot::new(1);
    let result = update(&slot, |n| {
        if n == 1 {
            slot.store(50);
        }
        (n < 10).then_some(n + 1)
    });
    assert_eq!(result, Err(50));
    assert_eq!(slot.load(), 50);

    // Threads race to take slots 1..=limit. Every slot is taken once, none twice.
    for (limit, threads) in [(0, 4), (1, 4), (1000, 8), (5000, 8)] {
        let counts = claim_all(limit, threads);
        assert_eq!(counts.len(), threads, "one count per thread");
        assert_eq!(counts.iter().sum::<usize>(), limit, "limit {limit}");
    }
    println!("all checks passed");
}
