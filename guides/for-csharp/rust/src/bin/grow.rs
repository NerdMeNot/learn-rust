fn main() {
    let mut xs = vec![1, 2, 3];
    for x in &xs {
        xs.push(x * 10);
    }
    println!("{xs:?}");
}
