fn main() {
    let mut v = vec![1, 2, 3, 4];
    for x in &v {
        if x % 2 == 0 {
            v.remove(0);
        }
    }
    println!("v: {v:?}");
}
