use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    println!("starting");
    let port: u16 = "http".parse()?;
    println!("port: {port}");
    Ok(())
}
