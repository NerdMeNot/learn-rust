fn main() {
    let mut xs = vec![1, 2, 3];
    let more: Vec<i32> = xs.iter().map(|x| x * 10).collect();
    xs.extend(more);
    println!("{xs:?}");
}
