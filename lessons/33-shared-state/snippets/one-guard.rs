use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let balance = Arc::new(Mutex::new(100));
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let balance = Arc::clone(&balance);
            thread::spawn(move || {
                // One guard covers the check and the change.
                let mut guard = balance.lock().unwrap();
                if *guard >= 60 {
                    *guard -= 60;
                    true
                } else {
                    false
                }
            })
        })
        .collect();
    let taken = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .filter(|&ok| ok)
        .count();
    println!("withdrawals that went through: {taken}");
    println!("balance: {}", balance.lock().unwrap());
}
