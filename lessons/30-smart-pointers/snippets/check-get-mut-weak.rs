use std::rc::Rc;

fn main() {
    let mut a = Rc::new(5);
    let w = Rc::downgrade(&a);
    assert!(Rc::get_mut(&mut a).is_none(), "a Weak blocks get_mut");
    drop(w);
    assert!(Rc::get_mut(&mut a).is_some());
}
