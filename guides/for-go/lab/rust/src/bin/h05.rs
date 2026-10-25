fn main() {
    let a = vec![1, 2, 3, 4];
    let mut b = a[..2].to_vec(); // an explicit copy of two items
    b.push(99);
    println!("{a:?} {b:?}");
}
