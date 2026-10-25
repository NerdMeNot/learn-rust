fn main() {
    let a = vec![1, 2, 3];
    let b = &a; // borrow: look at a, don't take it. No copy.
    println!("{a:?} {b:?}");
}
