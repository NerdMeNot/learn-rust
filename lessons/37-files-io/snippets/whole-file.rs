use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = std::env::temp_dir().join("lr-whole-file.txt");

    fs::write(&path, "milk\neggs\n")?;
    let text = fs::read_to_string(&path)?;
    println!("read_to_string: {text:?}");

    let bytes = fs::read(&path)?;
    println!("read: {} bytes, first four {:?}", bytes.len(), &bytes[..4]);

    fs::write(&path, "tea\n")?; // creates the file, or empties it first
    println!("after a second write: {:?}", fs::read_to_string(&path)?);

    fs::remove_file(&path)?;
    Ok(())
}
