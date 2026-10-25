use std::cell::Cell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

thread_local! {
    static COUNTER: Cell<u32> = const { Cell::new(0) };
}

fn main() {
    // park / unpark: sleep until another thread says so (loop: wake-ups can be spurious)
    let ready = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&ready);
    let waiter = thread::spawn(move || {
        while !flag.load(Ordering::Acquire) {
            thread::park();
        }
        "woken"
    });
    ready.store(true, Ordering::Release);
    waiter.thread().unpark();
    assert_eq!(waiter.join().unwrap(), "woken");

    // yield_now is a hint to the scheduler, with no visible result
    thread::yield_now();

    // is_finished is true once the closure has returned
    let done = thread::spawn(|| 1);
    while !done.is_finished() {
        thread::yield_now();
    }
    assert_eq!(done.join().unwrap(), 1);

    // an unnamed thread has no name; ids are unique per thread
    let id_here = thread::current().id();
    let id_there = thread::spawn(|| thread::current().id()).join().unwrap();
    assert_ne!(id_here, id_there);
    // thread_local!: every thread has its own copy, starting fresh
    COUNTER.set(10);
    let theirs = thread::spawn(|| {
        COUNTER.set(COUNTER.get() + 1);
        COUNTER.get()
    })
    .join()
    .unwrap();
    assert_eq!((theirs, COUNTER.get()), (1, 10));
    println!("others hold");
}
