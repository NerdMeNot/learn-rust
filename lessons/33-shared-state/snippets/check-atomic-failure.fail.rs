use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let hits = AtomicUsize::new(0);
    // A failed compare_exchange does not write, so its failure ordering cannot be Release.
    let _ = hits.compare_exchange(0, 1, Ordering::SeqCst, Ordering::Release);
}
