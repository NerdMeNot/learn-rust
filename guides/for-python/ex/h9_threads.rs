use std::sync::{Arc, Mutex};
use std::thread;

const N: u64 = 2_000_000;

fn add_one(n: u64) -> u64 {
    n + 1
}

fn main() {
    let count = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let count = Arc::clone(&count);
            thread::spawn(move || {
                for _ in 0..N {
                    let mut guard = count.lock().unwrap();
                    *guard = add_one(*guard);
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    println!("expected: {}", 4 * N);
    println!("got:      {}", *count.lock().unwrap());
}
