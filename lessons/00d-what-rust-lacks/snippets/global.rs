use std::sync::atomic::{AtomicU32, Ordering};

static VISITS: AtomicU32 = AtomicU32::new(0);

fn visit() -> u32 {
    VISITS.fetch_add(1, Ordering::Relaxed) + 1
}

fn main() {
    visit();
    visit();
    println!("visits: {}", visit());
    println!("loaded: {}", VISITS.load(Ordering::Relaxed));
}
