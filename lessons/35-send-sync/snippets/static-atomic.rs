use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

static HITS: AtomicU32 = AtomicU32::new(0);

fn main() {
    let handles: Vec<_> = (0..4)
        .map(|_| thread::spawn(|| HITS.fetch_add(1, Ordering::Relaxed)))
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("hits: {}", HITS.load(Ordering::Relaxed));
}
