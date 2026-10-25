#![forbid(unsafe_code)]

use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};

/// A person in a family tree. Parents own their children; a child only points back.
struct Person {
    name: String,
    parent: RefCell<Weak<Person>>,
    children: RefCell<Vec<Rc<Person>>>,
    dropped: Rc<Cell<u32>>, // shared count of how many Persons have been freed
}

impl Drop for Person {
    fn drop(&mut self) {
        self.dropped.set(self.dropped.get() + 1);
    }
}

fn person(name: &str, dropped: &Rc<Cell<u32>>) -> Rc<Person> {
    Rc::new(Person {
        name: name.to_string(),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
        dropped: Rc::clone(dropped),
    })
}

/// Make `child` one of `parent`'s children, and `parent` the parent of `child`.
fn add_child(parent: &Rc<Person>, child: &Rc<Person>) {
    parent.children.borrow_mut().push(Rc::clone(child));
    *child.parent.borrow_mut() = Rc::downgrade(parent);
}

/// The parent of `person`, if it is still alive.
fn parent_of(person: &Person) -> Option<Rc<Person>> {
    person.parent.borrow().upgrade()
}

/// How many children `person` has.
fn child_count(person: &Person) -> usize {
    person.children.borrow().len()
}

/// The names from the oldest known ancestor down to this person, joined with '/'.
fn path(person: &Person) -> String {
    match parent_of(person) {
        Some(parent) => format!("{}/{}", path(&parent), person.name),
        None => person.name.clone(),
    }
}

fn main() {
    let dropped = Rc::new(Cell::new(0));
    let root = person("root", &dropped);
    let mid = person("mid", &dropped);
    let leaf = person("leaf", &dropped);

    add_child(&root, &mid);
    add_child(&mid, &leaf);

    // the parent owns the child; the child must not own the parent
    assert_eq!(Rc::strong_count(&root), 1);
    assert_eq!(Rc::weak_count(&root), 1);
    assert_eq!(Rc::strong_count(&mid), 2);
    assert_eq!(Rc::strong_count(&leaf), 2);

    assert_eq!(path(&root), "root");
    assert_eq!(path(&mid), "root/mid");
    assert_eq!(path(&leaf), "root/mid/leaf");
    assert_eq!(child_count(&root), 1);
    assert_eq!(
        parent_of(&leaf).map(|p| p.name.clone()),
        Some("mid".to_string())
    );

    // dropping the root frees only the root: `mid` is still held by this function
    drop(root);
    assert_eq!(dropped.get(), 1);
    assert!(parent_of(&mid).is_none());
    assert_eq!(path(&leaf), "mid/leaf");

    // dropping the last owner of each frees the rest, and nothing is left over
    drop(mid);
    assert_eq!(dropped.get(), 2);
    drop(leaf);
    assert_eq!(dropped.get(), 3, "every person must be freed");

    println!("all checks passed");
}
