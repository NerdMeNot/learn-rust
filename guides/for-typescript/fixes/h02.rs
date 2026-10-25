fn main() {
    let a = vec![1, 2, 3];
    let mut b = a.clone(); // a separate copy of the vector
    b.push(4);
    println!("{a:?} {b:?}");
}
