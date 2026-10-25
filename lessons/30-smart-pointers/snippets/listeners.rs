use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // what the listeners saw; every listener must be able to add to it
    let seen = Rc::new(RefCell::new(Vec::new()));

    let mut listeners: Vec<Box<dyn Fn(&str)>> = Vec::new();
    for name in ["audit", "mail"] {
        let seen = Rc::clone(&seen); // each closure owns a share
        listeners.push(Box::new(move |event| {
            seen.borrow_mut().push(format!("{name}: {event}"));
        }));
    }

    for event in ["login", "logout"] {
        for listener in &listeners {
            listener(event);
        }
    }

    println!("owners: {}", Rc::strong_count(&seen));
    println!(
        "size_of RefCell<Vec<String>>: {}",
        size_of::<RefCell<Vec<String>>>()
    );
    for line in seen.borrow().iter() {
        println!("{line}");
    }
}
