use std::fs;

fn main() -> Result<(), std::io::Error> {
    println!("reading config");
    let text = fs::read_to_string("settings.toml")?;
    println!("{} bytes", text.len());
    Ok(())
}
