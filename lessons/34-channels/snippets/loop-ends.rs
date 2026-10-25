use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Two workers, each with its own clone. We join them, so both are finished.
    let workers: Vec<_> = (1..=2)
        .map(|id| {
            let tx = tx.clone();
            thread::spawn(move || tx.send(format!("from worker {id}")).unwrap())
        })
        .collect();
    for w in workers {
        w.join().unwrap();
    }

    // The workers are gone, but main still holds `tx`.
    let mut msgs = vec![rx.try_recv().unwrap(), rx.try_recv().unwrap()];
    msgs.sort();
    println!("queued messages: {msgs:?}");
    println!("try_recv, tx alive: {:?}", rx.try_recv());
    println!(
        "recv_timeout, tx alive: {:?}",
        rx.recv_timeout(Duration::from_millis(50))
    );
    // `for msg in rx` would wait here for ever: a sender exists that nobody will use.

    drop(tx);
    println!("try_recv, tx dropped: {:?}", rx.try_recv());
    println!("recv, tx dropped: {:?}", rx.recv());
    assert_eq!(rx.try_recv(), Err(TryRecvError::Disconnected));
}
