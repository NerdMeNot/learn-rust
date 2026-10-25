use std::sync::mpsc;
use std::thread;

fn main() {
    // tx is the sending end, rx the receiving end. T is inferred from the first send.
    let (tx, rx) = mpsc::channel();

    let worker = thread::spawn(move || {
        for i in 1..=3 {
            tx.send(format!("job {i}")).unwrap();
        }
        // tx is dropped here, when the closure ends
    });

    for _ in 0..3 {
        println!("recv: {:?}", rx.recv());
    }
    worker.join().unwrap();

    // Nothing is queued and the only sender is gone, so waiting would be pointless.
    println!("recv after the worker ended: {:?}", rx.recv());
}
