use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::sync::{Arc, Barrier, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

// A job is any closure that can run once on another thread.
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Pool {
    // an Option so that finish() can take the Sender out and drop it while the workers still run
    jobs: Option<mpsc::Sender<Job>>,
    workers: Vec<JoinHandle<()>>,
}

impl Pool {
    /// Starts `size` worker threads. They all take jobs from one channel.
    fn new(size: usize) -> Pool {
        let (tx, rx) = mpsc::channel::<Job>();
        let rx = Arc::new(Mutex::new(rx));
        let workers = (0..size)
            .map(|_| {
                let rx = Arc::clone(&rx);
                thread::spawn(move || {
                    loop {
                        // the guard is a temporary: it is released before the job runs
                        let job = rx.lock().unwrap().recv();
                        match job {
                            Ok(job) => job(),
                            Err(_) => break, // every Sender is gone and the queue is empty
                        }
                    }
                })
            })
            .collect();
        Pool {
            jobs: Some(tx),
            workers,
        }
    }

    /// Queues a job for any free worker.
    fn run<F: FnOnce() + Send + 'static>(&self, job: F) {
        self.jobs.as_ref().unwrap().send(Box::new(job)).unwrap();
    }

    /// Accepts no more jobs, lets the queued ones finish, and waits for the workers.
    fn finish(self) {
        drop(self.jobs); // the last Sender: recv() now fails once the queue is empty
        for worker in self.workers {
            worker.join().unwrap();
        }
    }
}

// Gives up when `f` takes too long: a hung pool is the likeliest bug.
fn within<T: Send + 'static>(what: &str, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = tx.send(f());
    });
    match rx.recv_timeout(Duration::from_secs(20)) {
        Ok(value) => value,
        Err(RecvTimeoutError::Timeout) => {
            panic!(
                "{what}: still waiting after 20 s. Is a lock held while waiting, or a Sender alive?"
            )
        }
        Err(RecvTimeoutError::Disconnected) => panic!("{what}: the function panicked"),
    }
}

fn main() {
    // every job runs exactly once, and finish() returns only after they have all run
    let count = Arc::new(AtomicUsize::new(0));
    let counted = Arc::clone(&count);
    within("100 jobs", move || {
        let pool = Pool::new(4);
        for _ in 0..100 {
            let count = Arc::clone(&counted);
            pool.run(move || {
                count.fetch_add(1, Ordering::SeqCst);
            });
        }
        pool.finish();
    });
    assert_eq!(count.load(Ordering::SeqCst), 100);

    // no jobs at all
    within("no jobs", || Pool::new(3).finish());

    // Two jobs that each wait for the other: they only finish if two workers are inside
    // a job AT THE SAME TIME. A worker that keeps the lock while it runs a job fails here.
    let done = Arc::new(AtomicUsize::new(0));
    let done2 = Arc::clone(&done);
    within("two jobs at once", move || {
        let pool = Pool::new(2);
        let meet = Arc::new(Barrier::new(2));
        for _ in 0..2 {
            let (meet, done) = (Arc::clone(&meet), Arc::clone(&done2));
            pool.run(move || {
                meet.wait();
                done.fetch_add(1, Ordering::SeqCst);
            });
        }
        pool.finish();
    });
    assert_eq!(done.load(Ordering::SeqCst), 2);

    // a pool of 3 uses 3 threads, whatever the number of jobs, and never the caller's
    let main_id = thread::current().id();
    let ids = Arc::new(Mutex::new(HashSet::new()));
    let ids2 = Arc::clone(&ids);
    within("thread count", move || {
        let pool = Pool::new(3);
        for _ in 0..60 {
            let ids = Arc::clone(&ids2);
            pool.run(move || {
                ids.lock().unwrap().insert(thread::current().id());
            });
        }
        pool.finish();
    });
    let ids = ids.lock().unwrap();
    assert!(!ids.contains(&main_id), "a job ran on the calling thread");
    assert!(
        ids.len() <= 3,
        "{} threads ran jobs; the pool has 3",
        ids.len()
    );

    println!("all checks passed");
}
