use std::sync::{Mutex, mpsc};

fn main() {
    let (tx, rx) = mpsc::channel();
    for job in 1..=3 {
        tx.send(job).unwrap();
    }
    drop(tx);
    let rx = Mutex::new(rx);

    // The guard is dropped at the semicolon: the lock is free while we work.
    let job = rx.lock().unwrap().recv().unwrap();
    println!(
        "let: job {job}, lock free during the job: {}",
        rx.try_lock().is_ok()
    );

    // In `while let` the guard lives as long as the loop body: nobody else can take a job.
    while let Ok(job) = rx.lock().unwrap().recv() {
        println!(
            "while let: job {job}, lock free during the job: {}",
            rx.try_lock().is_ok()
        );
    }
}
