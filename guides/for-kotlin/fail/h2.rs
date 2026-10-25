use std::num::ParseIntError;

fn parse_age(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn main() {
    let n: i32 = parse_age("42");
    println!("{}", n + 1);
}
