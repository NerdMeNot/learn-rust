use std::cell::RefCell;
use std::rc::Rc;

struct Counter {
    hits: u32,
}

fn main() {
    let a = Rc::new(RefCell::new(Counter { hits: 0 }));
    let b = Rc::clone(&a);

    a.borrow_mut().hits += 1;
    b.borrow_mut().hits += 1;
    println!("hits: {}", a.borrow().hits);
}
