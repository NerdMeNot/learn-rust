use std::sync::atomic::{AtomicUsize, Ordering};

// One counter for the whole test program: every test in this file shares it.
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

fn new_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

#[test]
fn creates_alice() {
    assert_eq!(new_id(), 1);
}

#[test]
fn creates_bob() {
    assert_eq!(new_id(), 1);
}
