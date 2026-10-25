use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..4 {
        let results = Arc::clone(&results);
        handles.push(thread::spawn(move || {
            for _ in 0..100_000 {
                results.lock().unwrap().push(i * 10);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{} items (expected 400000)", results.lock().unwrap().len());
}
