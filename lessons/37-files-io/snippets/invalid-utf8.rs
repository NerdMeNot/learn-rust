use std::fs::{self, File};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("lr-invalid-utf8.txt");
    fs::write(&path, b"good\nbad \xff here\nlast\n")?; // 0xff is never valid UTF-8

    let err = fs::read_to_string(&path).unwrap_err();
    println!("read_to_string: {err} (kind {:?})", err.kind());

    let bytes = fs::read(&path)?;
    println!("read + lossy: {:?}", String::from_utf8_lossy(&bytes));

    for line in BufReader::new(File::open(&path)?).lines() {
        match line {
            Ok(line) => println!("lines: Ok({line:?})"),
            Err(e) => println!("lines: Err({:?})", e.kind()),
        }
    }

    fs::remove_file(&path)?;
    Ok(())
}
