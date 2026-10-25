#![forbid(unsafe_code)]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

/// Runs every job on a thread of its own, all at the same time, and returns the results in
/// the order of `jobs`. Threads are started first and joined afterwards.
///
/// The bounds are the part to get right: what must hold for `F` and `T` so that the jobs
/// and their results may cross to another thread?
fn run_all<T, F>(jobs: Vec<F>) -> Vec<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let handles: Vec<_> = jobs.into_iter().map(|job| thread::spawn(job)).collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    // results come back in job order, and jobs may own data (a String each)
    let names = ["ann", "bo", "cyd"];
    let jobs: Vec<_> = names
        .iter()
        .map(|name| {
            let name = name.to_string();
            move || format!("hello {name}")
        })
        .collect();
    assert_eq!(run_all(jobs), ["hello ann", "hello bo", "hello cyd"]);

    // no jobs, no threads
    let none: Vec<fn() -> u32> = Vec::new();
    assert!(run_all(none).is_empty());

    // The jobs must really run at the same time: each one waits (a bounded number of
    // polite spins) until all four have started. Run one after another, the first job
    // gives up waiting and reports false.
    let arrived = Arc::new(AtomicUsize::new(0));
    let jobs: Vec<_> = (0..4)
        .map(|_| {
            let arrived = Arc::clone(&arrived);
            move || {
                arrived.fetch_add(1, Ordering::SeqCst);
                for _ in 0..200_000 {
                    if arrived.load(Ordering::SeqCst) == 4 {
                        return true;
                    }
                    thread::yield_now();
                }
                false
            }
        })
        .collect();
    assert_eq!(
        run_all(jobs),
        [true, true, true, true],
        "the four jobs must run concurrently, not one after another"
    );

    // a job that runs on another thread really is on another thread
    let main_id = thread::current().id();
    let jobs: Vec<_> = (0..3).map(|_| move || thread::current().id()).collect();
    for id in run_all(jobs) {
        assert_ne!(id, main_id);
    }

    println!("all checks passed");
}
