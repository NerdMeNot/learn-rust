use std::sync::mpsc::{self, RecvTimeoutError, SyncSender};
use std::thread;
use std::time::Duration;

/// Sends 0, 2, 4, ... on `tx`, one value at a time, until the receiver hangs up.
/// Returns how many sends succeeded, and the value that could not be delivered.
fn count_up(tx: SyncSender<u64>) -> (usize, Option<u64>) {
    todo!(
        "loop over 0.. ; send(i * 2) returns Result: on Err(e) the undelivered value is e.0, so stop and return it. Do not unwrap"
    )
}

// Gives up when `f` takes too long: a producer that ignores the error never stops.
fn within<T: Send + 'static>(what: &str, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(Duration::from_secs(20)) {
        Ok(value) => value,
        Err(RecvTimeoutError::Timeout) => {
            panic!("{what}: still running after 20 s. Does the loop stop when send fails?")
        }
        Err(RecvTimeoutError::Disconnected) => {
            panic!("{what}: the function panicked (did it unwrap a failed send?)")
        }
    }
}

fn main() {
    // Nobody is listening from the start: nothing is sent, and the first value bounces.
    let (tx, rx) = mpsc::sync_channel(0);
    drop(rx);
    assert_eq!(within("no receiver", move || count_up(tx)), (0, Some(0)));

    // A rendezvous channel (bound 0) hands each value straight to the receiver. After
    // the receiver has taken five and gone, the sixth send is the one that fails.
    let (tx, rx) = mpsc::sync_channel(0);
    let producer = thread::spawn(move || count_up(tx));
    let got: Vec<u64> = rx.iter().take(5).collect();
    drop(rx);
    assert_eq!(got, [0, 2, 4, 6, 8]);
    let result = within("producer", move || producer.join().unwrap());
    assert_eq!(result, (5, Some(10)));

    // the same with a bound of 2: the producer may be up to two values ahead
    let (tx, rx) = mpsc::sync_channel(2);
    let producer = thread::spawn(move || count_up(tx));
    let got: Vec<u64> = rx.iter().take(3).collect();
    drop(rx);
    assert_eq!(got, [0, 2, 4]);
    let (sent, bounced) = within("producer", move || producer.join().unwrap());
    assert!((3..=5).contains(&sent), "sent {sent}");
    assert_eq!(bounced, Some(2 * sent as u64));

    println!("all checks passed");
}
