fn main() {
    let v = vec![10, 20, 30, 40];
    let all: &[i32] = &v;
    let tail: &[i32] = &v[1..];

    println!("all: len {}", all.len());
    println!("tail: len {}", tail.len());
    println!(
        "tail starts {} bytes after all (one i32 is {} bytes)",
        tail.as_ptr() as usize - all.as_ptr() as usize,
        size_of::<i32>()
    );
    println!(
        "each is {} bytes: an address and a length",
        size_of::<&[i32]>()
    );
    println!(
        "no copy: all starts where the Vec's elements start: {}",
        all.as_ptr() == v.as_ptr()
    );
}
