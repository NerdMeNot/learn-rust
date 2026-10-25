// `original` is the address of the caller's buffer, so we can tell whether
// `xs` is that same buffer or a copy of it.
fn report(label: &str, xs: &[u8], original: *const u8) {
    println!("{label}:");
    println!("  len: {}", xs.len());
    println!("  same buffer as the caller's: {}", xs.as_ptr() == original);
}

fn main() {
    let data = vec![1u8; 10_000_000];
    let original = data.as_ptr();

    // Borrow: the function gets an address and a length (16 bytes).
    report("borrowed", &data, original);

    // Clone: allocate 10 MB more and copy every byte into it first.
    let copy = data.clone();
    report("cloned", &copy, original);
    println!("the copy has its own capacity: {}", copy.capacity());

    println!("a reference to the data: {} bytes", size_of::<&[u8]>());
    println!("data is still ours: {}", data.len());
}
