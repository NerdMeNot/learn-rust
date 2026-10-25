use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{LazyLock, OnceLock};
use std::thread;

static BUILDS: AtomicUsize = AtomicUsize::new(0);
static CONFIG: OnceLock<u32> = OnceLock::new();
static TABLE: LazyLock<HashMap<&str, u32>> = LazyLock::new(|| {
    BUILDS.fetch_add(1, Ordering::SeqCst);
    HashMap::from([("answer", 42)])
});

fn main() {
    let handles: Vec<_> = (0..8)
        .map(|_| {
            thread::spawn(|| {
                let a = *CONFIG.get_or_init(|| {
                    BUILDS.fetch_add(1, Ordering::SeqCst);
                    7
                });
                a + TABLE["answer"]
            })
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), 49);
    }
    // One initializer for the OnceLock and one for the LazyLock, however many threads asked.
    assert_eq!(BUILDS.load(Ordering::SeqCst), 2);
    println!("each initializer ran once");
}
