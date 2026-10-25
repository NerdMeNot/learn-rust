fn main() {
    let count: i32 = 3;
    let price: f64 = 2.5;
    let total = count * price; // no silent int-to-float conversion
    println!("{total}");
}
