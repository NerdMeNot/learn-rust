#![forbid(unsafe_code)]
#![allow(dead_code)]

use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread::{self, ThreadId};

// `is_sync!(T)` asks the compiler whether `T: Sync` and gives back a bool.
macro_rules! is_sync {
    ($t:ty) => {{
        struct Probe<T: ?Sized>(PhantomData<T>);
        trait No {
            fn answer(&self) -> bool {
                false
            }
        }
        impl<T: ?Sized> No for Probe<T> {}
        impl<T: ?Sized + Sync> Probe<T> {
            fn answer(&self) -> bool {
                true
            }
        }
        Probe::<$t>(PhantomData).answer()
    }};
}

/// Counts hits. Several threads must be able to call `hit` on one shared `Counter`.
struct Counter {
    hits: AtomicU32,
    /// Which threads called `hit`. It is already shared safely: leave it alone.
    threads: Mutex<HashSet<ThreadId>>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            hits: AtomicU32::new(0),
            threads: Mutex::new(HashSet::new()),
        }
    }

    fn hit(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
        self.threads.lock().unwrap().insert(thread::current().id());
    }

    fn total(&self) -> u32 {
        self.hits.load(Ordering::Relaxed)
    }

    fn threads_seen(&self) -> usize {
        self.threads.lock().unwrap().len()
    }
}

/// Starts `threads` scoped threads that each call `counter.hit()` `times` times, and
/// waits for all of them.
fn hit_from_threads(counter: &Counter, threads: usize, times: u32) {
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                for _ in 0..times {
                    counter.hit();
                }
            });
        }
    });
}

fn main() {
    let counter = Counter::new();
    hit_from_threads(&counter, 8, 1000);
    assert_eq!(
        counter.threads_seen(),
        8,
        "the hits must come from 8 different threads"
    );
    assert_eq!(counter.total(), 8000, "no hit may be lost");
    assert!(
        is_sync!(Counter),
        "Counter must be Sync to be shared by reference"
    );

    // a second counter is separate from the first
    let other = Counter::new();
    other.hit();
    assert_eq!(other.total(), 1);
    assert_eq!(other.threads_seen(), 1);
    assert_eq!(counter.total(), 8000);

    // the main thread can still use a counter after the threads are done
    counter.hit();
    assert_eq!(counter.total(), 8001);

    // zero threads do nothing; zero hits per thread do nothing
    hit_from_threads(&other, 0, 5);
    hit_from_threads(&other, 3, 0);
    assert_eq!(other.total(), 1);
    hit_from_threads(&other, 3, 2);
    assert_eq!(other.total(), 7);
    assert_eq!(other.threads_seen(), 4);

    println!("all checks passed");
}
