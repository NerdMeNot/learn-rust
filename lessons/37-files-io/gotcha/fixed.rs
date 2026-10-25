use std::fs::{self, File};
use std::io::Read; // brings read_to_string (and read, bytes, ...) into scope

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("rbc-hello.txt");
    fs::write(&path, "hello")?;

    let mut f = File::open(&path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    println!("{text}");
    fs::remove_file(&path)?;
    Ok(())
}
