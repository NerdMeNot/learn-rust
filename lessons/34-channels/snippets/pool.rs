use std::sync::{Arc, Mutex, mpsc};
use std::thread;

fn main() {
    let (job_tx, job_rx) = mpsc::channel::<u64>();
    let (result_tx, result_rx) = mpsc::channel();

    // A Receiver cannot be cloned (single consumer), so the workers share one behind a lock.
    let job_rx = Arc::new(Mutex::new(job_rx));

    let workers: Vec<_> = (0..4)
        .map(|_| {
            let job_rx = Arc::clone(&job_rx);
            let result_tx = result_tx.clone();
            thread::spawn(move || {
                let mut handled = 0;
                loop {
                    // The guard is a temporary: it is released at the end of this statement,
                    // so other workers can take the lock while this one handles its job.
                    let job = job_rx.lock().unwrap().recv();
                    let Ok(n) = job else { break }; // closed and empty
                    result_tx.send((n, n * n)).unwrap();
                    handled += 1;
                }
                handled
            })
        })
        .collect();
    drop(result_tx); // workers hold the other clones

    for n in 1..=20 {
        job_tx.send(n).unwrap();
    }
    drop(job_tx); // no more jobs: each worker's recv will return Err

    let mut results: Vec<(u64, u64)> = result_rx.iter().collect();
    results.sort(); // arrival order depends on the scheduler
    let handled: usize = workers.into_iter().map(|w| w.join().unwrap()).sum();

    println!("jobs handled by all workers: {handled}");
    println!("first three results: {:?}", &results[..3]);
    println!(
        "sum of squares: {}",
        results.iter().map(|r| r.1).sum::<u64>()
    );
}
