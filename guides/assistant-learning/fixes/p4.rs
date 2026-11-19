fn size(n: i32) -> &'static str {
    match n {
        ..=-1 => "negative",
        0 => "zero",
        1..=9 => "small",
        10.. => "large",
    }
}

fn main() {
    println!("{}", size(42));
}
