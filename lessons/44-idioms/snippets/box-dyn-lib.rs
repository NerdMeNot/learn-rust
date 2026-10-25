use std::error::Error;
use std::num::ParseIntError;

/// A library function whose signature says only "some error".
pub fn read_port(text: &str) -> Result<u16, Box<dyn Error>> {
    let (key, value) = text.split_once('=').ok_or("no '=' in the line")?;
    if key != "port" {
        return Err(format!("unexpected key {key:?}").into());
    }
    Ok(value.parse::<u16>()?)
}

fn main() {
    for text in ["port=80", "port=eighty", "host=db", "port"] {
        match read_port(text) {
            Ok(p) => println!("{text}: {p}"),
            Err(e) => {
                // To react to one kind of failure, the caller has to guess the concrete type.
                if e.downcast_ref::<ParseIntError>().is_some() {
                    println!("{text}: bad number ({e})");
                } else {
                    println!("{text}: other ({e})");
                }
            }
        }
    }
}
