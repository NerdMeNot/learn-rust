use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use tokio::sync::oneshot;

// A rayon job that works through 5 chunks. After chunk 1 it tells us, then waits for a
// go-ahead (this only forces the order: the receiver is dropped while the job is mid-way).
// `polite` jobs look at `tx.is_closed()` before each chunk; the others do not.
async fn run(polite: bool) -> usize {
    let done = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = oneshot::channel::<usize>();
    let (halfway_tx, halfway) = oneshot::channel::<()>();
    let (finished_tx, finished) = oneshot::channel::<()>();
    let (go_tx, go) = mpsc::channel::<()>(); // the job blocks on this one, which is fine on a rayon thread

    let counter = Arc::clone(&done);
    let mut halfway_tx = Some(halfway_tx);
    rayon::spawn(move || {
        for _ in 0..5 {
            if polite && tx.is_closed() {
                break;
            }
            counter.fetch_add(1, Ordering::SeqCst); // "process" one chunk
            if let Some(tx) = halfway_tx.take() {
                tx.send(()).unwrap();
                go.recv().unwrap();
            }
        }
        let sent = tx.send(5);
        println!(
            "  job over: sending the result {}",
            if sent.is_ok() {
                "worked"
            } else {
                "failed, nobody is listening"
            }
        );
        finished_tx.send(()).unwrap();
    });

    halfway.await.unwrap(); // the job has finished chunk 1
    drop(rx); // the async side gives up, like a request that was cancelled
    go_tx.send(()).unwrap();
    finished.await.unwrap(); // wait for the job to end, so the count is final
    done.load(Ordering::SeqCst)
}

#[tokio::main]
async fn main() {
    println!("job that ignores cancellation:");
    let a = run(false).await;
    println!("  chunks processed: {a}");
    println!("job that checks tx.is_closed():");
    let b = run(true).await;
    println!("  chunks processed: {b}");
    assert_eq!((a, b), (5, 1));
}
