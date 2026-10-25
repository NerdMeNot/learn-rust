mod count;
mod words;

use std::env;
use std::fs;
use std::io;

fn main() -> Result<(), io::Error> {
    let path = env::args().nth(1).expect("usage: wordcount FILE");
    let text = fs::read_to_string(&path)?;
    let counts = count::count_words(&text);
    let total: usize = counts.values().sum();
    println!("{total} words, {} distinct", counts.len());
    println!(
        "\"the\" appears {} times",
        counts.get("the").copied().unwrap_or(0)
    );
    Ok(())
}
