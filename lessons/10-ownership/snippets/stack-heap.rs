fn main() {
    let n: u64 = 7;
    let boxed: Box<[u8; 4096]> = Box::new([0; 4096]);
    let big: Vec<u64> = vec![7; 1_000_000];

    // size_of_val(&x) is the size of x itself: the part on the stack.
    println!(
        "n:     {} bytes on the stack, nothing on the heap",
        size_of_val(&n)
    );
    println!(
        "big:   {} bytes on the stack, {} bytes on the heap",
        size_of_val(&big),
        big.len() * size_of::<u64>()
    );
    println!(
        "boxed: {} bytes on the stack, {} bytes on the heap",
        size_of_val(&boxed),
        size_of_val(&*boxed)
    );

    let heap_before = big.as_ptr();
    let moved = big; // copies the 24-byte header, not the 8 MB
    println!(
        "same heap data after the move: {}",
        moved.as_ptr() == heap_before
    );
    println!("len {}, capacity {}", moved.len(), moved.capacity());
}
