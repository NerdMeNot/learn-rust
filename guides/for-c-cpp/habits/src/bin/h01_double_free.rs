fn main() {
    let a = String::from("hello");
    let b = a;
    drop(a);
    drop(b);
}
