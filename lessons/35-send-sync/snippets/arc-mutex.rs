use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let total = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let total = Arc::clone(&total);
            thread::spawn(move || *total.lock().unwrap() += 1)
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("total: {}", total.lock().unwrap());
}
