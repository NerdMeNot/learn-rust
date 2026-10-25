use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    let shared = Arc::new(RefCell::new(0));
    let other = Arc::clone(&shared);
    thread::spawn(move || *other.borrow_mut() += 1)
        .join()
        .unwrap();
    println!("{}", shared.borrow());
}
