use std::cell::RefCell;
use std::rc::Rc;

type Log = Rc<RefCell<Vec<String>>>;

fn note(log: &Log, text: String) {
    log.borrow_mut().push(text);
}

/// Adds a follow-up note for every note so far.
fn add_follow_ups(log: &Log) {
    for line in log.borrow().iter() {
        note(log, format!("follow up on {line}"));
    }
}

fn main() {
    let log: Log = Rc::new(RefCell::new(vec!["call Ada".to_string()]));
    add_follow_ups(&log);
    println!("{:?}", log.borrow());
}
