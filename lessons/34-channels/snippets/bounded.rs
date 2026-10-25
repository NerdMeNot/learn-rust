use std::sync::mpsc::{self, TrySendError};

fn main() {
    // channel(): the queue grows. Sending never waits for the receiver.
    let (tx, rx) = mpsc::channel();
    for i in 0..100_000 {
        tx.send(i).unwrap();
    }
    println!(
        "channel(): 100000 sends done, {} waiting",
        rx.try_iter().count()
    );

    // sync_channel(2): room for two messages. try_send says "full" instead of waiting.
    let (tx, rx) = mpsc::sync_channel(2);
    println!("try_send 1: {:?}", tx.try_send(1));
    println!("try_send 2: {:?}", tx.try_send(2));
    println!("try_send 3: {:?}", tx.try_send(3));
    println!("recv: {:?}", rx.recv());
    println!("try_send 3 again: {:?}", tx.try_send(3));
    if let Err(TrySendError::Full(value)) = tx.try_send(4) {
        println!("full: the value {value} is handed back");
    }

    // sync_channel(0): a rendezvous. A send completes only when a receiver takes it.
    let (tx, _rx) = mpsc::sync_channel(0);
    println!("try_send 1 on sync_channel(0): {:?}", tx.try_send(1));
}
