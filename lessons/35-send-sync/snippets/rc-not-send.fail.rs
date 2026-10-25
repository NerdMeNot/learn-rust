use std::rc::Rc;

fn assert_send<T: Send>() {}

fn main() {
    assert_send::<Rc<i32>>();
}
