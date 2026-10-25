use std::sync::Mutex;
use std::thread;

fn main() {
    let total = Mutex::new(0);
    thread::scope(|s| {
        s.spawn(|| *total.lock().unwrap() += 1); // the thread takes the lock itself
    });
    println!("total: {}", total.lock().unwrap());
}
