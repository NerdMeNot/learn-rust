/// Split a colour written as 0xRRGGBB into its (red, green, blue) bytes.
fn rgb(colour: u32) -> (u8, u8, u8) {
    // `as u8` throws away every bit above the lowest 8, so the stray top byte disappears.
    ((colour >> 16) as u8, (colour >> 8) as u8, colour as u8)
}

fn main() {
    assert_eq!(rgb(0xff8800), (255, 136, 0));
    assert_eq!(rgb(0x000000), (0, 0, 0));
    assert_eq!(rgb(0x123456), (0x12, 0x34, 0x56));
    // The case to watch: a stray top byte (alpha) must not leak into the result.
    assert_eq!(rgb(0xff_12_34_56), (0x12, 0x34, 0x56));
    println!("all checks passed");
}
