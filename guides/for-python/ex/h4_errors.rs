use std::num::ParseIntError;

fn parse_port(text: &str) -> Result<u16, ParseIntError> {
    text.parse()
}

fn main() {
    for text in ["8080", "http"] {
        match parse_port(text) {
            Ok(port) => println!("{port}"),
            Err(e) => println!("bad port: {e}"),
        }
    }
}
