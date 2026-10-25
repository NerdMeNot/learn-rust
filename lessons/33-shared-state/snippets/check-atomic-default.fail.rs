use std::sync::atomic::AtomicUsize;

fn main() {
    let hits = AtomicUsize::new(0);
    hits.fetch_add(1); // C++ `hits++` means seq_cst; Rust has no default ordering
}
