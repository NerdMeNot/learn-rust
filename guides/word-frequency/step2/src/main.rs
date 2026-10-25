mod words;

use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = env::args().nth(1).expect("usage: wordcount FILE");
    let text = fs::read_to_string(&path)?;
    let found = words::words(&text);
    let first = &found[..found.len().min(8)];
    println!("{} words; the first eight: {first:?}", found.len());
    Ok(())
}
