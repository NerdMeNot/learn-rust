mod colors {
    pub const RED: u32 = 1;
    pub const GREEN: u32 = 2;
}

use colors::*;

// An item written in this module beats a glob import of the same name.
const GREEN: u32 = 99;

fn main() {
    println!("RED: {RED}");
    println!("GREEN: {GREEN}");
    println!("colors::GREEN: {}", colors::GREEN);
}
