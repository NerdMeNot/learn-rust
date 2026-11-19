/// The device takes a brightness level from 0 to 255 in one byte.
fn brightness_byte(level: i32) -> u8 {
    level as u8
}

fn main() {
    for level in [200, 300, -1] {
        println!("level {level} -> byte {}", brightness_byte(level));
    }
}
