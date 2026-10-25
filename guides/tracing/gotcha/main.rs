use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    // Meant to hold the lock for the rest of main.
    let _ = counter.lock().unwrap();

    println!("lock is held: {}", counter.try_lock().is_err());
}
