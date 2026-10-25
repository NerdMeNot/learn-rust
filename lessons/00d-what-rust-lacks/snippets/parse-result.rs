use std::num::ParseIntError;

fn double(text: &str) -> Result<i32, ParseIntError> {
    let n = text.parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    println!("double 21: {:?}", double("21"));
    println!("double x: {:?}", double("x"));
}
