use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};

/// Counts words in anything that can be read line by line.
fn count_words<R: BufRead>(reader: R) -> io::Result<usize> {
    let mut n = 0;
    for line in reader.lines() {
        n += line?.split_whitespace().count();
    }
    Ok(n)
}

/// Writes a report to anything that can be written to.
fn report<W: Write>(out: &mut W, name: &str, n: usize) -> io::Result<()> {
    writeln!(out, "{name}: {n} words")
}

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("lr-traits.txt");
    let text = "one two\nthree\n";
    fs::write(&path, text)?;

    println!("from a byte slice: {}", count_words(text.as_bytes())?);
    println!(
        "from a file: {}",
        count_words(BufReader::new(File::open(&path)?))?
    );

    let mut memory: Vec<u8> = Vec::new();
    report(&mut memory, "vec", 3)?;
    println!("into a Vec<u8>: {:?}", String::from_utf8(memory).unwrap());

    let mut file = File::create(&path)?;
    report(&mut file, "file", 3)?;
    println!("into a File: {:?}", fs::read_to_string(&path)?);

    report(&mut io::stdout(), "stdout", 3)?;

    fs::remove_file(&path)?;
    Ok(())
}
