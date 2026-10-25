use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = env::args().nth(1).expect("usage: wordcount FILE");
    let text = fs::read_to_string(&path)?;
    println!(
        "{path}: {} bytes, {} lines",
        text.len(),
        text.lines().count()
    );
    Ok(())
}
