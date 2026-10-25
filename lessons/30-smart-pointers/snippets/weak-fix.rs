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
    let leaf = node("leaf");
    println!(
        "leaf parent: {:?}",
        leaf.parent.borrow().upgrade().map(|p| p.name)
    );
    {
        let root = node("root");
        root.children.borrow_mut().push(Rc::clone(&leaf)); // the parent owns the child
        *leaf.parent.borrow_mut() = Rc::downgrade(&root); // the child only points back
        println!(
            "root: strong {}, weak {}",
            Rc::strong_count(&root),
            Rc::weak_count(&root)
        );
        println!(
            "leaf: strong {}, weak {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
        println!(
            "leaf parent: {:?}",
            leaf.parent.borrow().upgrade().map(|p| p.name)
        );
    } // root goes: its count was 1, so it is dropped now
    println!(
        "leaf parent: {:?}",
        leaf.parent.borrow().upgrade().map(|p| p.name)
    );
    println!("leaf: strong {}", Rc::strong_count(&leaf));
}
