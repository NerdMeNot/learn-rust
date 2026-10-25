fn main() {
    let mut v = vec![1, 2];
    let w = vec![3, 4];
    v.extend(w);
    println!("{v:?} {w:?}");
}
