// Backs the table of what each end can do (never shown on the page).
use std::cell::Cell;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender, SyncSender};

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn assert_clone<T: Clone>() {}

fn main() {
    assert_clone::<Sender<Rc<i32>>>(); // cloning a sender needs nothing of T
    assert_clone::<SyncSender<Rc<i32>>>();

    assert_send::<Sender<String>>();
    assert_sync::<Sender<String>>();
    assert_send::<SyncSender<String>>();
    assert_sync::<SyncSender<String>>();
    assert_send::<Receiver<String>>();
    assert_send::<Sender<Cell<i32>>>(); // Cell<i32> is Send, so the sender is too
    println!("ok");
}
