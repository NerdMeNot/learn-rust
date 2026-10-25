fn main() {
    let a = vec![1, 2, 3];
    let b = a;
    println!("b: {b:?}");
    println!("a: {a:?}"); // a was moved into b
}
