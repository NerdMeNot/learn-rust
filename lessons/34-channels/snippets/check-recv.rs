// Backs the table of receiving calls (never shown on the page).
use std::sync::mpsc::{self, RecvError, RecvTimeoutError, TryRecvError};
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    // queue empty, a sender exists
    assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
    assert_eq!(
        rx.recv_timeout(Duration::from_millis(20)),
        Err(RecvTimeoutError::Timeout)
    );
    assert_eq!(rx.try_iter().next(), None);

    // queue not empty: every call returns the message
    tx.send(1).unwrap();
    assert_eq!(rx.try_recv(), Ok(1));
    tx.send(2).unwrap();
    assert_eq!(rx.recv_timeout(Duration::from_millis(20)), Ok(2));
    tx.send(3).unwrap();
    assert_eq!(rx.recv(), Ok(3));
    tx.send(4).unwrap();
    assert_eq!(rx.iter().next(), Some(4));
    tx.send(7).unwrap();
    tx.send(8).unwrap();
    assert_eq!(rx.try_iter().collect::<Vec<_>>(), [7, 8]);

    // messages sent before the last sender is dropped are still delivered
    tx.send(5).unwrap();
    tx.send(6).unwrap();
    tx.send(9).unwrap();
    drop(tx);
    assert_eq!(rx.recv(), Ok(5));
    assert_eq!(rx.try_recv(), Ok(6));
    assert_eq!(rx.try_iter().next(), Some(9)); // try_iter after the last Sender is gone

    // queue empty, every sender gone
    assert_eq!(rx.recv(), Err(RecvError));
    assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
    assert_eq!(
        rx.recv_timeout(Duration::from_millis(20)),
        Err(RecvTimeoutError::Disconnected)
    );
    assert_eq!(rx.iter().next(), None);
    assert_eq!(rx.try_iter().next(), None);
    assert_eq!(rx.into_iter().count(), 0);

    // `for m in rx` (into_iter) hands out the queued messages, then ends
    let (tx, rx) = mpsc::channel::<i32>();
    tx.send(1).unwrap();
    tx.send(2).unwrap();
    drop(tx);
    assert_eq!(rx.into_iter().collect::<Vec<_>>(), [1, 2]);
    println!("ok");
}
