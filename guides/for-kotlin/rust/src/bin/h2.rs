use std::num::ParseIntError;

fn parse_age(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse()
}

fn total(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(parse_age(a)? + parse_age(b)?)
}

fn main() {
    println!("{:?}", total("4", "5"));
    match total("4", "x") {
        Ok(n) => println!("{n}"),
        Err(e) => println!("error: {e}"),
    }
    println!("{}", total("4", "x").is_err());
}
