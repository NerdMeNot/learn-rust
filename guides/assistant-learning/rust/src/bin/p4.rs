fn size(n: i32) -> &'static str {
    match n {
        0 => "zero",
        x if x < 0 => "negative",
        1..=9 => "small",
        x if x >= 10 => "large",
    }
}

fn main() {
    println!("{}", size(42));
}
