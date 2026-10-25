fn main() {
    let a = vec![1, 2, 3];
    let mut b = a; // moves the vector: b owns it now
    b.push(4);
    println!("{a:?}");
}
