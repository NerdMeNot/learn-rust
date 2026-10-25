fn main() {
    let mut a = vec![7, 8, 9];
    let p = &a[0];
    a.push(10); // may move the elements to a new buffer
    println!("p: {p}");
}
