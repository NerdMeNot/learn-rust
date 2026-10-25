use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;

// How many sends have returned by the time the receiver has taken `taken` messages?
// With a bound of `cap` the producer cannot be more than `cap` ahead.
fn run(cap: usize) {
    let (tx, rx) = mpsc::sync_channel(cap);
    let done = Arc::new(AtomicUsize::new(0));
    let producer = {
        let done = Arc::clone(&done);
        thread::spawn(move || {
            for i in 0..1000 {
                tx.send(i).unwrap();
                done.fetch_add(1, Ordering::SeqCst); // this send has returned
            }
        })
    };
    let mut taken = 0;
    for _ in rx {
        taken += 1;
        assert!(done.load(Ordering::SeqCst) <= taken + cap);
    }
    producer.join().unwrap();
    println!("sync_channel({cap}): never more than {cap} ahead of the receiver");
}

fn main() {
    for cap in [0, 1, 4] {
        run(cap);
    }

    // channel() has no limit: the producer finishes before anyone receives.
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || (0..1000).for_each(|i| tx.send(i).unwrap()))
        .join()
        .unwrap();
    println!("channel(): producer finished 1000 sends, receiver has taken 0");
    println!("then the receiver finds {}", rx.try_iter().count());
}
