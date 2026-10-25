fn size(n: u8) -> &'static str {
    match n {
        0..=9 => "small",
        10..=99 => "medium",
    }
}

fn main() {
    println!("{}", size(42));
}
