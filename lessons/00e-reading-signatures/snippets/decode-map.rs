fn main() {
    // T = &str (what the Option holds), U = usize (what the closure returns)
    let length = Some("hello").map(|s| s.len());
    let nothing = None::<&str>.map(|s| s.len());
    println!("length: {length:?}");
    println!("nothing: {nothing:?}");
}
