use std::sync::mpsc::{self, Sender};

fn wants_sender(_tx: Sender<i32>) {}

fn main() {
    let (tx, _rx) = mpsc::channel::<i32>();
    let _ = tx.try_send(1); // try_send exists only on SyncSender

    let (stx, _srx) = mpsc::sync_channel::<i32>(1);
    wants_sender(stx); // Sender and SyncSender are different types
}
