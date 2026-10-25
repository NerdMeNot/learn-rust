use std::fs::{self, File};

fn main() -> std::io::Result<()> {
    let path = std::env::temp_dir().join("rbc-hello.txt");
    fs::write(&path, "hello")?;

    let mut f = File::open(&path)?;
    let mut text = String::new();
    f.read_to_string(&mut text)?;
    println!("{text}");
    Ok(())
}
