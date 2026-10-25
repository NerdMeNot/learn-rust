use std::num::ParseIntError;

/// The caller gets the failure as a value and decides what it means.
pub fn parse_port(text: &str) -> Result<u16, ParseIntError> {
    text.trim().parse()
}

fn main() {
    for text in ["8080", "http"] {
        match parse_port(text) {
            Ok(port) => println!("{text}: port {port}"),
            Err(e) => println!("{text}: {e}, using 80"),
        }
    }
}
