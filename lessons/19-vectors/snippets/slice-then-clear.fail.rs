fn main() {
    let mut v = vec![3, 9, 4];
    let tail = &v[1..];
    v.clear();
    println!("tail is {tail:?}");
}
