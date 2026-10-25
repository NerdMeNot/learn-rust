use std::ffi::{CStr, CString};

fn main() {
    // Rust to C: a Rust string may hold a 0 byte, but a C string ends at the first one.
    match CString::new("ab\0cd") {
        Ok(_) => println!("built a CString"),
        Err(e) => println!("CString::new failed: {e}"),
    }
    let owned = CString::new("héllo").unwrap();
    println!("bytes with NUL: {:?}", owned.as_bytes_with_nul());

    // C to Rust: borrowing C's bytes. UTF-8 is a promise C does not make.
    let latin1 = CStr::from_bytes_with_nul(b"caf\xe9\0").unwrap();
    println!("len without NUL: {}", latin1.count_bytes());
    println!("to_str: {:?}", latin1.to_str());
    println!("to_string_lossy: {}", latin1.to_string_lossy());

    // A slice with no NUL is rejected, not read past its end.
    println!("no NUL: {:?}", CStr::from_bytes_with_nul(b"abc"));
}
