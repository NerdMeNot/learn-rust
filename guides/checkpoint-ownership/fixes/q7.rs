fn main() {
    let mut v = vec![1, 2, 3];
    for i in 0..v.len() {
        if v[i] == 2 {
            v.push(v[i] * 10);
        }
    }
    println!("{v:?}");
}
