use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));

    let m2 = Arc::clone(&m);
    let first = thread::spawn(move || {
        let _guard = m2.lock().unwrap();
        panic!("first thread failed");
    })
    .join();
    println!("first thread failed: {}", first.is_err());

    let m3 = Arc::clone(&m);
    let second = thread::spawn(move || {
        *m3.lock().unwrap() += 1;
    })
    .join();
    println!("second thread failed too: {}", second.is_err());
}
