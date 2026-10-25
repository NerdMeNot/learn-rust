use std::sync::Mutex;

fn main() {
    let counter = Mutex::new(0);

    // A name, even one starting with an underscore, keeps the guard alive to the end of main.
    let _guard = counter.lock().unwrap();

    println!("lock is held: {}", counter.try_lock().is_err());
}
