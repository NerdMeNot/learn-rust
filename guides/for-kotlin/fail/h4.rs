fn main() {
    let mut xs = vec![1, 2, 3];
    let view: &[i32] = &xs;
    xs.push(4);
    println!("{:?}", view);
}
