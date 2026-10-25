use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    let total = Arc::new(RefCell::new(0));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let total = Arc::clone(&total);
        handles.push(thread::spawn(move || *total.borrow_mut() += 1));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{}", total.borrow());
}
