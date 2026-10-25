use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

fn main() {
    let words = vec!["alpha", "beta", "gamma"];
    let hits = AtomicU32::new(0);
    thread::scope(|s| {
        for w in &words {
            // borrows `words` and `hits`: both are Sync, and the scope ends before they do
            s.spawn(|| hits.fetch_add(w.len() as u32, Ordering::Relaxed));
        }
    });
    println!("letters: {}", hits.load(Ordering::Relaxed));
}
