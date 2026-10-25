use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let lock = Arc::new(Mutex::new(0));
    let released = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel();

    let (l, r) = (Arc::clone(&lock), Arc::clone(&released));
    let holder = thread::spawn(move || {
        let guard = l.lock().unwrap();
        tx.send(()).unwrap(); // the lock is held now
        thread::sleep(Duration::from_millis(100));
        r.store(true, Ordering::SeqCst); // set before the guard goes
        drop(guard);
    });

    rx.recv().unwrap();
    assert!(lock.try_lock().is_err(), "held by the other thread");
    let _second = lock.lock().unwrap(); // waits for its turn
    assert!(
        released.load(Ordering::SeqCst),
        "lock() returned only after the release"
    );
    holder.join().unwrap();
}
