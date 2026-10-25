use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let log = Rc::new(RefCell::new(vec![String::from("start")]));
    let also_log = Rc::clone(&log);

    // the borrow() lasts for the whole loop
    for line in log.borrow().iter() {
        println!("saw: {line}");
        also_log.borrow_mut().push(format!("after {line}"));
    }
}
