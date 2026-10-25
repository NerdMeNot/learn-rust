use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn assert_send<T: Send>() {}

// Shared<T> holds a T behind an Arc<Mutex<..>>.
struct Shared<T> {
    inner: Arc<Mutex<T>>,
}

fn main() {
    assert_send::<Shared<i32>>(); // fine: Send when T is Send
    assert_send::<Shared<Rc<i32>>>(); // not fine: the T inside is an Rc
}
