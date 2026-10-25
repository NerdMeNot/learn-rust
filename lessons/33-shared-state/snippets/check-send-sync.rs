use std::cell::{Cell, RefCell};
use std::sync::atomic::AtomicUsize;
use std::sync::{Arc, Mutex, RwLock};

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    // Arc<T> crosses threads when T is Send and Sync.
    assert_send::<Arc<Vec<i32>>>();
    assert_send::<Arc<AtomicUsize>>();
    assert_send::<Arc<Mutex<Vec<i32>>>>();
    assert_send::<Arc<RwLock<Vec<i32>>>>();
    // A Mutex is Sync when T is Send, even when T is not Sync.
    assert_sync::<Mutex<Cell<i32>>>();
    assert_send::<Arc<Mutex<RefCell<i32>>>>();
    // A RwLock needs both.
    assert_sync::<RwLock<Vec<i32>>>();
    println!("all assertions compiled");
}
