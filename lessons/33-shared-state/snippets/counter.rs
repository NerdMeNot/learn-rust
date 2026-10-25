use std::sync::{Arc, Mutex};
use std::thread;

fn count_with_threads(threads: usize, per_thread: usize) -> usize {
    let total = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..threads)
        .map(|_| {
            let total = Arc::clone(&total);
            thread::spawn(move || {
                for _ in 0..per_thread {
                    *total.lock().unwrap() += 1;
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    let n = *total.lock().unwrap();
    n
}

fn main() {
    println!("8 threads x 1000: {}", count_with_threads(8, 1000));
    for round in 0..200 {
        assert_eq!(count_with_threads(8, 1000), 8000, "round {round}");
    }
    println!("200 more rounds: every total was 8000");
}
