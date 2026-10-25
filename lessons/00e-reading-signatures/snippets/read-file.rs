use std::error::Error;
use std::fs;

fn read(path: &str) -> Result<String, Box<dyn Error>> {
    let text = fs::read_to_string(path)?; // an io::Error becomes a Box<dyn Error>
    Ok(text)
}

fn main() {
    match read("/no/such/file") {
        Ok(text) => println!("read {} bytes", text.len()),
        Err(e) => println!("error: {e}"),
    }
}
