use std::cell::RefCell;

fn main() {
    let list = RefCell::new(vec![1, 2]);
    let first = list.borrow_mut();
    println!("first borrow is alive: {}", first.len());
    let second = list.borrow_mut(); // compiles; fails when it runs
    println!("never printed: {}", second.len());
}
