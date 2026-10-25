fn main() {
    let n = 10;
    let a = &n;
    let b = &n;
    let c = a; // a shared reference can be copied
    println!("a: {a}, b: {b}, c: {c}, n: {n}");
}
