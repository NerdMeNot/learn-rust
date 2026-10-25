use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let log = Rc::new(RefCell::new(vec![String::from("start")]));
    let also_log = Rc::clone(&log);

    // copy the lines out; the borrow ends at the semicolon
    let lines: Vec<String> = log.borrow().clone();
    for line in &lines {
        println!("saw: {line}");
        also_log.borrow_mut().push(format!("after {line}"));
    }
    println!("log: {:?}", log.borrow());
}
