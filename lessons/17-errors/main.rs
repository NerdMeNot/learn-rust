use std::num::ParseIntError;

fn parse_age(s: &str) -> Result<u32, ParseIntError> {
    let n: u32 = s.parse()?;   // ? = if this failed, return the error now
    Ok(n)
}

fn main() {
    match parse_age("42") {
        Ok(n) => println!("age {n}"),
        Err(e) => println!("error: {e}"),
    }
    match parse_age("abc") {
        Ok(n) => println!("age {n}"),
        Err(e) => println!("error: {e}"),
    }
}
