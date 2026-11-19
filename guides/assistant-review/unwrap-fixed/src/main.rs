mod config {
    use std::num::ParseIntError;

    /// Reads a port number such as "8080".
    pub fn parse_port(text: &str) -> Result<u16, ParseIntError> {
        text.trim().parse()
    }
}

fn main() {
    for input in ["8080", "80a", "70000"] {
        match config::parse_port(input) {
            Ok(port) => println!("{input:?} -> {port}"),
            Err(e) => println!("{input:?} -> not a port: {e}"),
        }
    }
}
