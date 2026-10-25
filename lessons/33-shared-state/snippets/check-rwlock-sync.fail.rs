use std::cell::Cell;
use std::sync::{Mutex, RwLock};

fn assert_sync<T: Sync>() {}

fn main() {
    assert_sync::<Mutex<Cell<i32>>>();
    assert_sync::<RwLock<Cell<i32>>>();
}
