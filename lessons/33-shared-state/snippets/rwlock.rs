use std::sync::{Arc, Barrier, RwLock};
use std::thread;

fn main() {
    let lock = Arc::new(RwLock::new(String::from("v1")));

    // Two readers that both hold a read guard when they reach the barrier.
    // If readers excluded each other, the second could never get in and this would not finish.
    let barrier = Arc::new(Barrier::new(2));
    let readers: Vec<_> = (0..2)
        .map(|_| {
            let (lock, barrier) = (Arc::clone(&lock), Arc::clone(&barrier));
            thread::spawn(move || {
                let guard = lock.read().unwrap();
                barrier.wait();
                guard.len()
            })
        })
        .collect();
    for r in readers {
        println!("a reader saw {} bytes", r.join().unwrap());
    }

    let reader = lock.read().unwrap();
    println!(
        "reader alive, try_write fails: {}",
        lock.try_write().is_err()
    );
    println!("reader alive, try_read works: {}", lock.try_read().is_ok());
    drop(reader);
    println!("reader gone, try_write works: {}", lock.try_write().is_ok());

    lock.write().unwrap().push_str("+edit");
    println!("value: {}", lock.read().unwrap());
}
