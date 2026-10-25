use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // one list with two owners: a count of owners, and no copy of the list
    let log = Rc::new(RefCell::new(Vec::new()));
    let other = Rc::clone(&log);

    log.borrow_mut().push("from log");
    other.borrow_mut().push("from other");

    println!("log: {}", log.borrow().join(", "));
    println!("same list: {}", Rc::ptr_eq(&log, &other));
}
