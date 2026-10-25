use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Node {
    name: &'static str,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop {}", self.name);
    }
}

fn node(name: &'static str) -> Rc<Node> {
    Rc::new(Node {
        name,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    })
}

fn main() {
    let root = node("root");
    let leaf = node("leaf");
    root.children.borrow_mut().push(Rc::clone(&leaf));
    *leaf.parent.borrow_mut() = Rc::downgrade(&root);
    println!(
        "root owners: {}, leaf owners: {}",
        Rc::strong_count(&root),
        Rc::strong_count(&leaf)
    );
}
