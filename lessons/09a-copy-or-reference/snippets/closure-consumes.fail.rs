fn main() {
    let s = String::from("hi");
    let f = || drop(s); // the body takes `s`, so the closure takes it
    f();
    println!("{s}");
}
