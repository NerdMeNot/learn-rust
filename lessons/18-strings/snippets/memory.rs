fn main() {
    let s = String::from("héllo");
    let v: &str = &s[0..3];

    println!("s: len {} capacity {}", s.len(), s.capacity());
    println!("v: {v:?}, len {}", v.len());
    // Same address: v points into s's bytes. Nothing was copied.
    println!("same start: {}", v.as_ptr() == s.as_ptr());
    println!(
        "size on the stack: String {} bytes, &str {} bytes",
        size_of::<String>(),
        size_of::<&str>()
    );
}
