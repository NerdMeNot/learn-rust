use std::cell::RefCell;
use std::rc::Rc;

/// Adds a follow-up message after every urgent one.
fn flag_urgent(messages: &Rc<RefCell<Vec<String>>>) {
    for m in messages.borrow().iter() {
        if m.starts_with("URGENT") {
            messages.borrow_mut().push(format!("follow up: {m}"));
        }
    }
}

fn main() {
    let messages = Rc::new(RefCell::new(vec![
        "hello".to_string(),
        "URGENT: disk full".to_string(),
    ]));
    flag_urgent(&messages);
    println!("{:?}", messages.borrow());
}
