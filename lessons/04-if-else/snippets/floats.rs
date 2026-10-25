fn main() {
    let sum = 0.1 + 0.2;
    println!("0.1 + 0.2 == 0.3: {}", sum == 0.3);
    println!("sum: {sum}");
    println!("close enough: {}", (sum - 0.3_f64).abs() < 1e-9);

    let nan = f64::NAN;
    println!("nan == nan: {}", nan == nan);
    println!("nan < 0.0: {}", nan < 0.0);
    println!("nan > 0.0: {}", nan > 0.0);
    println!("nan >= 0.0: {}", nan >= 0.0);
    // both comparisons are false, so an `else` branch is not "the opposite"
    if nan < 0.0 {
        println!("negative");
    } else {
        println!("else branch: not negative, but not a number either");
    }
}
