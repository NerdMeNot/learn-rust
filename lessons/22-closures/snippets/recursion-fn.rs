fn main() {
    // A function item has a name, so it can call itself.
    fn factorial(n: u64) -> u64 {
        if n <= 1 { 1 } else { n * factorial(n - 1) }
    }
    println!("factorial(5): {}", factorial(5));
}
