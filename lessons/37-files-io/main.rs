use std::fs;
use std::io::{BufRead, BufReader, Write};

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("rbc-notes.txt");

    // write a whole file at once (creates it, or empties it first)
    fs::write(&path, "milk\neggs\n")?;

    // open for appending and add a line
    let mut f = fs::OpenOptions::new().append(true).open(&path)?;
    writeln!(f, "bread")?;

    // read the whole file into one String
    let all = fs::read_to_string(&path)?;
    println!("{} bytes", all.len());

    // read line by line, without loading the whole file
    let reader = BufReader::new(fs::File::open(&path)?);
    for (i, line) in reader.lines().enumerate() {
        let line = line?; // each line can fail on its own (bad UTF-8, disk error)
        println!("{}: {line}", i + 1);
    }

    println!("file name: {:?}", path.file_name().unwrap());

    // a missing file is an Err you can inspect, not a crash
    match fs::read_to_string("/no/such/file.txt") {
        Ok(_) => println!("found it?"),
        Err(e) => println!("error: {e} (kind {:?})", e.kind()),
    }

    fs::remove_file(&path)?;
    Ok(())
}
