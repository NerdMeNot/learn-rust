use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("Box<i32>: {}", size_of::<Box<i32>>());
    println!("Box<str>: {}", size_of::<Box<str>>());
    println!("Box<[i32]>: {}", size_of::<Box<[i32]>>());
    println!("Rc<str>: {}", size_of::<Rc<str>>());
    println!("Arc<str>: {}", size_of::<Arc<str>>());
    println!("Box<dyn Debug>: {}", size_of::<Box<dyn Debug>>());

    let name: Rc<str> = Rc::from("shared text"); // one allocation, no String capacity
    let other = Rc::clone(&name);
    println!("{name} / {other}, owners: {}", Rc::strong_count(&name));
}
