use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    let total = Arc::new(RefCell::new(0));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let total = Arc::clone(&total);
            thread::spawn(move || *total.borrow_mut() += 1)
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    println!("total: {}", total.borrow());
}
