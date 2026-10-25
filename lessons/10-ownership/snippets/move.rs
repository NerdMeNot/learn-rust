fn main() {
    let a = vec![1, 2, 3];
    let a_buffer = a.as_ptr(); // where a's numbers live on the heap

    let b = a; // move: b is the owner now. a can't be used again.

    println!("b: {b:?}");
    println!("b uses a's buffer: {}", b.as_ptr() == a_buffer);
}
