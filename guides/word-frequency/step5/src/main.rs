mod count;
mod report;
mod words;

use std::env;
use std::fs;
use std::io;

const TOP: usize = 10;

fn main() -> Result<(), io::Error> {
    let path = env::args().nth(1).expect("usage: wordcount FILE");
    let text = fs::read_to_string(&path)?;
    let counts = count::count_words(&text);
    let total: usize = counts.values().sum();
    let distinct = counts.len();
    let top = count::top_n(counts, TOP);
    report::print(total, distinct, &top);
    Ok(())
}
