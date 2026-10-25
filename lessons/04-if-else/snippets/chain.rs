fn describe(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n < 10 {
        "small"
    } else {
        "large"
    }
}

fn main() {
    for n in [-5, 0, 3, 10, 99] {
        println!("{n}: {}", describe(n));
    }
}
