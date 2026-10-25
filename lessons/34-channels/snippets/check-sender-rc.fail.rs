use std::rc::Rc;
use std::sync::mpsc::Sender;

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<Sender<Rc<i32>>>();
}
