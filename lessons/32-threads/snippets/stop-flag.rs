use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    let stop = AtomicBool::new(false);

    // the worker checks the flag; nothing else can make it stop
    let result = thread::scope(|s| {
        let worker = s.spawn(|| {
            while !stop.load(Ordering::Relaxed) {
                thread::yield_now(); // a real worker does a bit of work here
            }
            "stopped because the flag was set"
        });
        stop.store(true, Ordering::Relaxed);
        worker.join().unwrap()
    });
    println!("worker: {result}");
}
