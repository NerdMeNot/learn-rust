fn main() {
    let factorial = |n: u64| if n <= 1 { 1 } else { n * factorial(n - 1) };
    println!("{}", factorial(5));
}
