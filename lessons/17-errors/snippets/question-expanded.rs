use std::num::ParseIntError;

// The version with ?
fn short(text: &str) -> Result<u32, ParseIntError> {
    let n: u32 = text.parse()?;
    Ok(n + 1)
}

// What ? does, written out. The Err arm calls From::from on the error:
// when the types already match, that changes nothing.
fn long(text: &str) -> Result<u32, ParseIntError> {
    let n: u32 = match text.parse() {
        Ok(value) => value,
        Err(e) => return Err(From::from(e)),
    };
    Ok(n + 1)
}

fn main() {
    for text in ["41", "forty-one"] {
        println!("short({text:?}): {:?}", short(text));
        println!("long({text:?}): {:?}", long(text));
    }
}
