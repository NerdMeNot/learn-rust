use std::error::Error;

fn parse_port(text: &str) -> Result<u16, std::num::ParseIntError> {
    text.parse()
}

// main may return a Result. An Err ends the program with a message and a
// non-zero exit code. Here every step works, so it prints and returns Ok.
fn main() -> Result<(), Box<dyn Error>> {
    let port = parse_port("8080")?;
    println!("port: {port}");
    Ok(())
}
