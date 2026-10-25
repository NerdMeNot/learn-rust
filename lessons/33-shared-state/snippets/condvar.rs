use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let worker = {
        let pair = Arc::clone(&pair);
        thread::spawn(move || {
            println!("worker: setting the flag");
            let (flag, cvar) = &*pair;
            *flag.lock().unwrap() = true;
            cvar.notify_one();
        })
    };

    let (flag, cvar) = &*pair;
    // wait_while keeps sleeping while the closure returns true.
    let guard = cvar
        .wait_while(flag.lock().unwrap(), |ready| !*ready)
        .unwrap();
    println!("main: woke up, flag is {}", *guard);
    drop(guard);
    worker.join().unwrap();

    // The notification can come before anyone waits. The flag is why that is fine.
    let done = Arc::new((Mutex::new(false), Condvar::new()));
    let d = Arc::clone(&done);
    thread::spawn(move || {
        *d.0.lock().unwrap() = true;
        d.1.notify_one();
    })
    .join()
    .unwrap();
    let guard = done
        .1
        .wait_while(done.0.lock().unwrap(), |ready| !*ready)
        .unwrap();
    println!("late waiter: flag was already {}", *guard);
}
