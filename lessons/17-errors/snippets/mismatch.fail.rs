// The function says its error is a String, but parse gives a ParseIntError.
fn double(text: &str) -> Result<u32, String> {
    let n: u32 = text.parse()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?}", double("21"));
}
