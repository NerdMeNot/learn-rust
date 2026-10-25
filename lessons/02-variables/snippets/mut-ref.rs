fn main() {
    let mut v = vec![1];
    let r = &mut v; // `r` is not `mut`, but it points at a value that is
    r.push(2);
    println!("v: {v:?}");
}
