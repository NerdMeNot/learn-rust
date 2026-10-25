use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    // The barrier makes both threads read before either writes, so the lost update happens every run.
    let barrier = Arc::new(Barrier::new(2));

    let handles: Vec<_> = (0..2)
        .map(|_| {
            let (counter, barrier) = (Arc::clone(&counter), Arc::clone(&barrier));
            thread::spawn(move || {
                let seen = counter.load(Ordering::SeqCst);
                barrier.wait();
                counter.store(seen + 1, Ordering::SeqCst);
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!(
        "two increments, load then store: {}",
        counter.load(Ordering::SeqCst)
    );

    counter.store(0, Ordering::SeqCst);
    counter.fetch_add(1, Ordering::SeqCst);
    counter.fetch_add(1, Ordering::SeqCst);
    println!(
        "two increments, fetch_add: {}",
        counter.load(Ordering::SeqCst)
    );
}
