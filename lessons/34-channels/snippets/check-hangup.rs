// Backs the table "what the other side sees" (never shown on the page).
use std::sync::mpsc::{self, RecvError, SendError, Sender, TryRecvError, TrySendError};
use std::thread;

// A function that returns early still drops the Sender it owns.
fn early_exit(tx: Sender<i32>) -> Option<i32> {
    tx.send(1).ok()?;
    let nothing: Option<i32> = None;
    nothing?; // returns here
    tx.send(2).ok()?;
    Some(0)
}

fn main() {
    // one of several senders dropped: nothing changes for the receiver
    let (tx, rx) = mpsc::channel::<i32>();
    let tx2 = tx.clone();
    drop(tx);
    tx2.send(1).unwrap();
    assert_eq!(rx.try_recv(), Ok(1));

    // the last sender dropped: queued messages first, then every call reports the end
    tx2.send(10).unwrap();
    tx2.send(11).unwrap();
    drop(tx2);
    assert_eq!(rx.recv(), Ok(10));
    assert_eq!(rx.try_recv(), Ok(11));
    assert_eq!(rx.recv(), Err(RecvError));
    assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
    assert_eq!(rx.iter().next(), None);

    // the same when the owner returns early, or panics (the panic is caught by join)
    let (tx, rx) = mpsc::channel::<i32>();
    assert_eq!(early_exit(tx), None);
    assert_eq!(rx.iter().collect::<Vec<_>>(), [1]);
    std::panic::set_hook(Box::new(|_| {}));
    let (tx, rx) = mpsc::channel::<i32>();
    let worker = thread::spawn(move || {
        tx.send(1).unwrap();
        panic!("worker failed");
    });
    assert_eq!(rx.iter().collect::<Vec<_>>(), [1]);
    assert!(worker.join().is_err());

    // receiver dropped: send fails and hands the value back, on every clone of the sender
    let (tx1, rx) = mpsc::channel::<i32>();
    let tx2 = tx1.clone();
    drop(rx);
    assert_eq!(tx1.send(1), Err(SendError(1)));
    assert_eq!(tx2.send(2), Err(SendError(2)));

    // the same for a bounded channel
    let (stx, srx) = mpsc::sync_channel::<i32>(1);
    drop(srx);
    assert_eq!(stx.send(3), Err(SendError(3)));
    assert!(matches!(
        stx.try_send(4),
        Err(TrySendError::Disconnected(4))
    ));
    println!("ok");
}
