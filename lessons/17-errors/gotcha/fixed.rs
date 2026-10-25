fn main() {
    let n: u32 = "42".parse().unwrap_or(0); // 0 if it fails, chosen on purpose
    println!("{}", n + 1);
}
