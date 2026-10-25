fn main() {
    let code = 'a' as u32; // fine: a char becomes an integer
    let wrong = 'a' as f64; // not allowed
    println!("{code} {wrong}");
}
