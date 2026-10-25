use std::sync::atomic::{AtomicU32, Ordering};

fn next() -> u32 {
    static CALLS: AtomicU32 = AtomicU32::new(0); // a static inside a function
    CALLS.fetch_add(1, Ordering::Relaxed) + 1 // changes without `mut` or `unsafe`
}

fn main() {
    println!("calls: {} {} {}", next(), next(), next());
}
