fn main() {
    let mut v = Vec::with_capacity(10);
    v.push(1);
    let first = &v[0];
    v.push(2); // there is room, so nothing would move
    println!("first: {first}");
}
