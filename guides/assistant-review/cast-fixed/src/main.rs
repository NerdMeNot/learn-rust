use std::num::TryFromIntError;

/// The device takes a brightness level from 0 to 255 in one byte.
fn brightness_byte(level: i32) -> Result<u8, TryFromIntError> {
    u8::try_from(level)
}

fn main() {
    for level in [200, 300, -1] {
        match brightness_byte(level) {
            Ok(byte) => println!("level {level} -> byte {byte}"),
            Err(e) => println!("level {level} -> refused: {e}"),
        }
    }
}
