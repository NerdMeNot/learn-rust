use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

static DATA: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::scope(|s| {
        s.spawn(|| {
            DATA.store(42, Ordering::Relaxed);
            READY.store(true, Ordering::Release); // publishes what was written before it
        });
        s.spawn(|| {
            while !READY.load(Ordering::Acquire) {
                std::hint::spin_loop();
            }
            // This load saw the Release store, so the 42 is visible here.
            println!("data after ready: {}", DATA.load(Ordering::Relaxed));
        });
    });
}
