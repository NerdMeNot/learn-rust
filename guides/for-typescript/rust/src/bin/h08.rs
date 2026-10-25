use std::num::ParseIntError;

fn parse_port(s: &str) -> Result<u16, ParseIntError> {
    s.parse::<u16>()
}

fn double_port(s: &str) -> Result<u16, ParseIntError> {
    let port = parse_port(s)?; // on Err, return it to the caller
    Ok(port * 2)
}

fn main() {
    println!("{:?}", "abc".parse::<i32>());
    println!("{:?}", double_port("80"));
    match double_port("80x") {
        Ok(p) => println!("port {p}"),
        Err(e) => println!("caught: {e}"),
    }
}
