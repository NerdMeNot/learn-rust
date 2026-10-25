use std::sync::{Arc, Barrier, Mutex, TryLockError};
use std::thread;

type Account = Arc<Mutex<&'static str>>;

fn main() {
    let a: Account = Arc::new(Mutex::new("account A"));
    let b: Account = Arc::new(Mutex::new("account B"));
    let barrier = Arc::new(Barrier::new(2));

    // Each thread takes its first lock, waits until the other has too, then tries the second.
    let spawn = |first: Account, second: Account| {
        let barrier = Arc::clone(&barrier);
        thread::spawn(move || {
            let _first = first.lock().unwrap();
            barrier.wait();
            let blocked = matches!(second.try_lock(), Err(TryLockError::WouldBlock));
            barrier.wait(); // keep the first lock until the other thread has tried
            blocked
        })
    };
    let t1 = spawn(Arc::clone(&a), Arc::clone(&b));
    let t2 = spawn(Arc::clone(&b), Arc::clone(&a));
    println!(
        "thread 1 (A, then B) would block on B: {}",
        t1.join().unwrap()
    );
    println!(
        "thread 2 (B, then A) would block on A: {}",
        t2.join().unwrap()
    );
}
