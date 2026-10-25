use rayon::prelude::*;
use std::sync::{Arc, Mutex};

// One runtime thread, so "the runtime is busy" has a single meaning.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let log = Arc::new(Mutex::new(Vec::new()));

    // A heartbeat task: it records a tick, then lets the runtime run something else.
    let beat = Arc::clone(&log);
    let heartbeat = tokio::spawn(async move {
        for i in 1..=3 {
            beat.lock().unwrap().push(format!("tick {i}"));
            tokio::task::yield_now().await;
        }
    });
    tokio::task::yield_now().await; // let the heartbeat start

    // CPU-bound work, called straight from an async fn.
    let total: u64 = (0..1_000_000u64).into_par_iter().map(|n| n % 7).sum();
    log.lock()
        .unwrap()
        .push(format!("par_iter done, total {total}"));

    heartbeat.await.unwrap();
    let log = log.lock().unwrap();
    for entry in log.iter() {
        println!("{entry}");
    }
    assert_eq!(
        *log,
        ["tick 1", "par_iter done, total 2999997", "tick 2", "tick 3"]
    );
}
