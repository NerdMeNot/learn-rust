use std::rc::Rc;
use std::thread;

fn main() {
    // the closure captures nothing, but it hands an Rc back across the thread boundary
    let handle = thread::spawn(|| Rc::new(5));
    let value = handle.join().unwrap();
    println!("{value}");
}
