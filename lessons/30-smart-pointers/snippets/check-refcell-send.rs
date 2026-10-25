use std::cell::RefCell;
use std::thread;

fn main() {
    // moved to the thread, not shared: RefCell<i32> is Send
    let own = RefCell::new(1);
    let out = thread::spawn(move || {
        *own.borrow_mut() += 1;
        own.into_inner()
    })
    .join()
    .unwrap();
    assert_eq!(out, 2);
    println!("a RefCell can move to another thread");
}
