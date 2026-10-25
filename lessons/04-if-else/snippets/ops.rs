fn main() {
    let (a, b) = (3, 8);
    println!("a == b: {}", a == b);
    println!("a != b: {}", a != b);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a > 1 && b > 1: {}", a > 1 && b > 1);
    println!("a > 5 || b > 5: {}", a > 5 || b > 5);
    println!("!(a < b): {}", !(a < b));
    // `&` binds tighter than `==` in Rust, so no parentheses are needed here
    println!("a & 1 == 1: {}", a & 1 == 1);
}
