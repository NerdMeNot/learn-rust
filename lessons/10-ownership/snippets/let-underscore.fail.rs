use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);
    // Meant to hold the lock for the rest of the function. It doesn't.
    let _ = counter.lock().unwrap();
    println!("working while holding the lock (or so it seems)");
}
