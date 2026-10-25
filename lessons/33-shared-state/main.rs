use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 0..3 {
        let results = Arc::clone(&results);      // another owner, same Vec
        handles.push(thread::spawn(move || {
            results.lock().unwrap().push(i * 10);  // lock, push, unlock
        }));
    }
    for h in handles {
        h.join().unwrap();
    }

    let mut r = results.lock().unwrap();
    r.sort();
    println!("{:?}", *r);
}
