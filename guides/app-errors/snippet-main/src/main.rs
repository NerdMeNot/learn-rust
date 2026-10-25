use anyhow::{Context, Result};

fn main() -> Result<()> {
    let port: u16 = "eighty".parse().context("reading the port")?;
    println!("port {port}");
    Ok(())
}
