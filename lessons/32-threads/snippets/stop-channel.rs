use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    // the worker's loop ends when every sender is gone: the channel is closed
    let worker = thread::spawn(move || {
        let mut sum = 0;
        for job in rx {
            sum += job;
        }
        sum
    });

    for job in 1..=3 {
        tx.send(job).unwrap();
    }
    drop(tx); // closing the channel is the stop signal
    println!(
        "worker added up {} before it stopped",
        worker.join().unwrap()
    );
}
