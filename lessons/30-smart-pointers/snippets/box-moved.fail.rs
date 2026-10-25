fn main() {
    let a = Box::new(5);
    let b = a;
    println!("b: {b}");
    println!("a: {a}");
}
