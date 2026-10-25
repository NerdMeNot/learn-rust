fn main() {
    let small: i32 = 5;
    let big: u64 = small;
    let other: i64 = 5;
    println!("{big} {}", small == other);
}
