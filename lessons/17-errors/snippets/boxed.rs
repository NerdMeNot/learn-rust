use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
struct TooBig(u32);

impl fmt::Display for TooBig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is too big", self.0)
    }
}

impl Error for TooBig {}

// Box<dyn Error> holds any error. ? converts into it, whatever the type.
fn check(text: &str) -> Result<u32, Box<dyn Error>> {
    if text.is_empty() {
        return Err("nothing to check".into()); // a &str becomes an error
    }
    let n: u32 = text.parse()?; // a ParseIntError
    if n > 100 {
        return Err(Box::new(TooBig(n))); // our own error type
    }
    Ok(n)
}

fn main() {
    for text in ["42", "", "x", "500"] {
        match check(text) {
            Ok(n) => println!("{text:?} -> ok {n}"),
            Err(e) => {
                // The concrete type is hidden, but you can ask for it back.
                let is_parse = e.downcast_ref::<ParseIntError>().is_some();
                println!("{text:?} -> {e} (a ParseIntError: {is_parse})");
            }
        }
    }
}
