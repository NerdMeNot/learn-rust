fn main() {
    let mut v = vec![10, 20, 30];
    let (i, j) = (0, 2);

    let a = &mut v[i];
    let b = &mut v[j];
    std::mem::swap(a, b);

    println!("v: {v:?}");
}
