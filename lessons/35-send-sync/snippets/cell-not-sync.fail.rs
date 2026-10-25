use std::cell::Cell;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<Cell<i32>>(); // fine: a Cell may move to another thread
    assert_sync::<Cell<i32>>(); // two threads may not share one
}
