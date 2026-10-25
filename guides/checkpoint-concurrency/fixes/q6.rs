use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let list = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    for i in 0..3 {
        let list = Arc::clone(&list);
        handles.push(thread::spawn(move || list.lock().unwrap().push(i)));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let mut items = list.lock().unwrap().clone();
    items.sort();
    println!("{items:?}");
}
