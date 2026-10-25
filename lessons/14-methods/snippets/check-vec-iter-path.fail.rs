fn main() {
    let v = vec![1, 2, 3];
    let total: i32 = Vec::iter(&v).sum();
    println!("{total}");
}
