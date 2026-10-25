fn main() {
    let mut v = vec![1, 2];
    v.push(v.len()); // reads v while the push is on its way
    println!("after v.push(v.len()): {v:?}");

    let mut w = vec![5, 6];
    w.push(w[0]);
    println!("after w.push(w[0]):    {w:?}");
}
