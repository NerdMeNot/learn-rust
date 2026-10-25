use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: i32 = "21".parse()?;
    println!("{}", n * 2);
    let m: i32 = "2l".parse()?;
    println!("{}", m * 2);
    Ok(())
}
