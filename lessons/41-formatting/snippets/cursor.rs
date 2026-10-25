use std::io::{Cursor, Write};

fn main() {
    let mut small = [0u8; 8];
    let mut cursor = Cursor::new(&mut small[..]); // lets write! fill a byte buffer
    let result = write!(cursor, "{} each", "$19.99 and more");
    println!("result: {:?}", result.map_err(|e| e.to_string()));
    println!("kept: {:?}", String::from_utf8_lossy(&small));
}
