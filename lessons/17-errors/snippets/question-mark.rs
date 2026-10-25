use std::num::ParseIntError;

// Each ? means: if this is an Err, return that Err from the whole function now.
// If it is Ok, take the value out and carry on.
fn total_cents(dollars: &str, cents: &str) -> Result<u32, ParseIntError> {
    let d: u32 = dollars.parse()?;
    let c: u32 = cents.parse()?;
    Ok(d * 100 + c)
}

fn main() {
    println!("total_cents(\"12\", \"50\"): {:?}", total_cents("12", "50"));
    println!("total_cents(\"x\", \"50\"): {:?}", total_cents("x", "50"));
    println!("total_cents(\"12\", \"\"): {:?}", total_cents("12", ""));
}
