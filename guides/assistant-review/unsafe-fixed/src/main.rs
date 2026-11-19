/// Returns the first byte of a message, or `None` if it is empty.
fn first_byte(bytes: &[u8]) -> Option<u8> {
    if bytes.is_empty() {
        return None;
    }
    // SAFETY: `bytes` is not empty (checked just above), so index 0 is in bounds.
    Some(unsafe { *bytes.get_unchecked(0) })
}

/// The same job in safe code: `first` does the bounds check.
fn first_byte_safe(bytes: &[u8]) -> Option<u8> {
    bytes.first().copied()
}

fn main() {
    println!("first byte: {:?}", first_byte(b"hi"));
    println!("first byte: {:?}", first_byte(b""));
    println!("first byte: {:?}", first_byte_safe(b""));
}
