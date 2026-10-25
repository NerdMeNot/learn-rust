fn main() {
    let mut v = vec![1, 2, 3];
    let first = &mut v[0];
    let second = &mut v[1];
    *first += 10;
    *second += 10;
    println!("{v:?}");
}
