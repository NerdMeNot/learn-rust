use std::panic::{self, AssertUnwindSafe};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

fn main() {
    panic::set_hook(Box::new(|_| {})); // keep the expected panics quiet

    // a scoped thread that panics and is never joined: the scope itself panics
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        thread::scope(|s| {
            s.spawn(|| panic!("inside the scope"));
        });
    }));
    let payload = outcome.expect_err("the scope should have panicked");
    let text = payload.downcast_ref::<&str>().copied().unwrap_or("");
    assert_eq!(text, "a scoped thread panicked");

    // joined by hand: the scope returns normally and join holds the Err
    let joined = thread::scope(|s| s.spawn(|| panic!("inside the scope")).join().is_err());
    assert!(joined);
    // a panic does not cancel the sibling threads: the scope waits for them to finish
    let sibling_finished = AtomicBool::new(false);
    let outcome = panic::catch_unwind(AssertUnwindSafe(|| {
        thread::scope(|s| {
            s.spawn(|| panic!("inside the scope"));
            s.spawn(|| {
                thread::yield_now();
                sibling_finished.store(true, Ordering::SeqCst);
            });
        });
    }));
    assert!(outcome.is_err());
    assert!(sibling_finished.load(Ordering::SeqCst));
    println!("scope facts hold");
}
