use std::sync::mpsc::Receiver;

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<Receiver<i32>>();
}
