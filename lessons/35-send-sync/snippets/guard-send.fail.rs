use std::sync::Mutex;
use std::thread;

fn main() {
    let total = Mutex::new(0);
    thread::scope(|s| {
        let mut guard = total.lock().unwrap();
        s.spawn(move || *guard += 1); // hands the lock itself to another thread
    });
    println!("total: {}", total.lock().unwrap());
}
