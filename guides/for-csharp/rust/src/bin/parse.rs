use std::num::ParseIntError;

fn double(text: &str) -> Result<i32, ParseIntError> {
    Ok(text.parse::<i32>()? * 2)
}

fn main() {
    println!("{:?}", double("21"));
    match double("x") {
        Ok(n) => println!("{n}"),
        Err(e) => println!("error: {e}"),
    }
}
