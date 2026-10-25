use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(vec![1, 2, 3]));

    let m2 = Arc::clone(&m);
    let result = thread::spawn(move || {
        let mut guard = m2.lock().unwrap();
        guard.push(4);
        panic!("worker failed halfway");
    })
    .join();
    println!("join returned an error: {}", result.is_err());
    println!("is_poisoned: {}", m.is_poisoned());

    match m.lock() {
        Ok(_) => println!("lock: Ok"),
        Err(e) => println!("lock: Err({e})"),
    }

    let guard = m.lock().unwrap_or_else(|e| e.into_inner());
    println!("data after recovery: {:?}", *guard);
    drop(guard);
    println!("still poisoned: {}", m.is_poisoned());
    m.clear_poison();
    println!("after clear_poison: {}", m.is_poisoned());
}
