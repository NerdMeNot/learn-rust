use rayon::prelude::*;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));

    // The rayon job waits on this gate until the heartbeat has ticked three times.
    // That is only here to force an order; real code would not wait.
    let (open_gate, gate) = mpsc::channel::<()>();

    let beat = Arc::clone(&log);
    let heartbeat = tokio::spawn(async move {
        for i in 1..=3 {
            beat.lock().unwrap().push(format!("tick {i}"));
            tokio::task::yield_now().await;
        }
        open_gate.send(()).unwrap();
    });

    // The same work, handed to rayon. The async fn waits on a oneshot channel.
    let (tx, rx) = oneshot::channel();
    rayon::spawn(move || {
        gate.recv().unwrap();
        let total: u64 = (0..1_000_000u64).into_par_iter().map(|n| n % 7).sum();
        let _ = tx.send(total);
    });
    let total = rx.await.unwrap(); // yields: the heartbeat runs while the job waits
    log.lock()
        .unwrap()
        .push(format!("rayon job done, total {total}"));

    heartbeat.await.unwrap();
    let log = log.lock().unwrap();
    for entry in log.iter() {
        println!("{entry}");
    }
    assert_eq!(
        *log,
        [
            "tick 1",
            "tick 2",
            "tick 3",
            "rayon job done, total 2999997"
        ]
    );
}
