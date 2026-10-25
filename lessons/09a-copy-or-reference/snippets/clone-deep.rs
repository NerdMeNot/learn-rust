use std::ptr;
use std::rc::Rc;

#[derive(Clone)]
struct Team {
    name: String,
    members: Vec<String>,
}

fn main() {
    let a = Team {
        name: String::from("red"),
        members: vec![String::from("ana")],
    };
    let mut b = a.clone(); // copies the name, the Vec and every String in it
    b.members[0].push('!');
    println!("a: {} {:?}", a.name, a.members);
    println!("b: {} {:?}", b.name, b.members);
    println!(
        "same member buffer: {}",
        ptr::eq(a.members.as_ptr(), b.members.as_ptr())
    );

    // Sharing is something you ask for: Rc::clone copies the arrow, not the Team.
    let shared = Rc::new(a);
    let other = Rc::clone(&shared);
    println!("same Team: {}", Rc::ptr_eq(&shared, &other));
    println!("owners of the Team: {}", Rc::strong_count(&shared));
}
