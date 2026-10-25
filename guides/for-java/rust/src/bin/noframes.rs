fn main() -> Result<(), std::num::ParseIntError> {
    let port: u16 = "eighty".parse()?;
    println!("port {port}");
    Ok(())
}
