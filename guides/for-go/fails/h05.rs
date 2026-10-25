fn main() {
    let mut a = vec![1, 2, 3, 4];
    let b = &a[..2];
    a.push(99);
    println!("{b:?}");
}
