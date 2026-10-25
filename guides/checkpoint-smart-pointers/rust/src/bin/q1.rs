use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("shared"));
    let b = Rc::clone(&a);
    let w = Rc::downgrade(&a);
    println!("{} {}", Rc::strong_count(&a), Rc::weak_count(&a));
    drop(a);
    println!("{:?}", w.upgrade());
    drop(b);
    println!("{:?}", w.upgrade());
}
