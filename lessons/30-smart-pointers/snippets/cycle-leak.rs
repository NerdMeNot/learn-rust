use std::cell::RefCell;
use std::rc::Rc;

struct Node {
    name: &'static str,
    other: RefCell<Option<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}

fn node(name: &'static str) -> Rc<Node> {
    Rc::new(Node {
        name,
        other: RefCell::new(None),
    })
}

fn main() {
    let probe;
    {
        let a = node("a");
        let b = node("b");
        *a.other.borrow_mut() = Some(Rc::clone(&b)); // a owns b
        *b.other.borrow_mut() = Some(Rc::clone(&a)); // b owns a
        println!(
            "owners of a: {}, owners of b: {}",
            Rc::strong_count(&a),
            Rc::strong_count(&b)
        );
        probe = Rc::downgrade(&a);
    } // a and b go; each count falls from 2 to 1, not to 0
    println!(
        "after the block, a is still alive: {}",
        probe.upgrade().is_some()
    );
}
