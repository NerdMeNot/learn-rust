/// Returns the first byte of a message.
fn first_byte(bytes: &[u8]) -> u8 {
    unsafe { *bytes.get_unchecked(0) }
}

fn main() {
    println!("first byte: {}", first_byte(b"hi"));
}
