fn main() {
    let s = "héllo";
    let b = s.as_bytes(); // a view of the same memory: no copy, no unsafe
    println!("bytes: {b:?}");
    println!("same address: {}", s.as_ptr() == b.as_ptr());
    println!("size of &str: {}", size_of::<&str>());
}
