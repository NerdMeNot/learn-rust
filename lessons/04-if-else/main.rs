fn main() {
    let n = 7;
    if n % 2 == 0 {
        println!("{n} is even");
    } else {
        println!("{n} is odd");
    }

    // `if` gives back a value, so Rust has no `?:`
    let size = if n > 5 { "big" } else { "small" };
    println!("{size}");
}
