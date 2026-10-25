fn main() {
    let n = 7;

    // `if` as the value of a `let`
    let size = if n > 5 { "big" } else { "small" };
    println!("size: {size}");

    // `if` as an argument, with no temporary variable
    println!("parity: {}", if n % 2 == 0 { "even" } else { "odd" });

    // a block as a branch: its last expression is the value
    let label = if n > 5 {
        let doubled = n * 2;
        doubled + 1
    } else {
        0
    };
    println!("label: {label}");
}
