use std::fmt;
use std::num::ParseIntError;

/// The signature lists every way this can fail.
#[derive(Debug)]
pub enum PortError {
    NoEquals,
    WrongKey(String),
    BadNumber(ParseIntError),
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortError::NoEquals => write!(f, "no '=' in the line"),
            PortError::WrongKey(k) => write!(f, "unexpected key {k:?}"),
            PortError::BadNumber(e) => write!(f, "bad number: {e}"),
        }
    }
}

impl std::error::Error for PortError {}

impl From<ParseIntError> for PortError {
    fn from(e: ParseIntError) -> PortError {
        PortError::BadNumber(e)
    }
}

pub fn read_port(text: &str) -> Result<u16, PortError> {
    let (key, value) = text.split_once('=').ok_or(PortError::NoEquals)?;
    if key != "port" {
        return Err(PortError::WrongKey(key.to_string()));
    }
    Ok(value.parse::<u16>()?)
}

fn main() {
    for text in ["port=80", "port=eighty", "host=db", "port"] {
        match read_port(text) {
            Ok(p) => println!("{text}: {p}"),
            Err(PortError::BadNumber(e)) => println!("{text}: bad number ({e})"),
            Err(e) => println!("{text}: other ({e})"),
        }
    }
}
