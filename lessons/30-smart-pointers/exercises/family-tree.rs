#![forbid(unsafe_code)]

use std::cell::Cell;
use std::rc::{Rc, Weak};

/// A person in a family tree. Parents own their children; a child only points back.
struct Person {
    name: String,
    parent: Weak<Person>,
    children: Vec<Rc<Person>>,
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
        parent: Weak::new(),
        children: Vec::new(),
        dropped: Rc::clone(dropped),
    })
}

/// Make `child` one of `parent`'s children, and `parent` the parent of `child`.
/// You only hold `&Rc<Person>`, so the two fields above cannot be changed as they are:
/// first change their types (and `person`), then fill this in.
fn add_child(parent: &Rc<Person>, child: &Rc<Person>) {
    todo!(
        "wrap the fields in RefCell: RefCell<Weak<Person>> is a weak link you can point somewhere else through &"
    )
}

/// The parent of `person`, if it is still alive.
fn parent_of(person: &Person) -> Option<Rc<Person>> {
    todo!("upgrade the weak link")
}

/// How many children `person` has.
fn child_count(person: &Person) -> usize {
    todo!("the length of the children list")
}

/// The names from the oldest known ancestor down to this person, joined with '/'.
fn path(person: &Person) -> String {
    todo!("walk up with parent_of, then join the names top-down")
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
