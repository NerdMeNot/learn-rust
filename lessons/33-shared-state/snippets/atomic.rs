use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

static HITS: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let handles: Vec<_> = (0..8)
        .map(|_| {
            thread::spawn(|| {
                for _ in 0..1000 {
                    HITS.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("hits: {}", HITS.load(Ordering::Relaxed));

    let old = HITS.fetch_add(5, Ordering::SeqCst);
    println!("fetch_add returned the old value: {old}");
    println!(
        "compare_exchange(8005, 0): {:?}",
        HITS.compare_exchange(8005, 0, Ordering::SeqCst, Ordering::SeqCst)
    );
    println!(
        "compare_exchange(8005, 1): {:?}",
        HITS.compare_exchange(8005, 1, Ordering::SeqCst, Ordering::SeqCst)
    );
    // try_update retries its closure until the swap succeeds, or stops when the closure says None.
    println!(
        "try_update(add 1 below 5): {:?}",
        HITS.try_update(Ordering::SeqCst, Ordering::SeqCst, |n| (n < 5)
            .then_some(n + 1))
    );
    println!(
        "fetch_max(9) returned the old value: {}",
        HITS.fetch_max(9, Ordering::SeqCst)
    );
}
