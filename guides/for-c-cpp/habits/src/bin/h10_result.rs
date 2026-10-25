use std::num::ParseIntError;

fn double(text: &str) -> Result<i32, ParseIntError> {
    let n: i32 = text.parse()?;
    Ok(n * 2)
}

fn main() -> Result<(), ParseIntError> {
    println!("{:?}", double("21"));
    println!("{:?}", double("abc"));
    let n = double("abc")?;
    println!("not reached: {n}");
    Ok(())
}
