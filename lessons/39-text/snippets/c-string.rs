use std::ffi::CString;

fn main() {
    let text = "a,b";
    let piece = text.split(',').next().unwrap();

    // A piece is a pointer and a length. The byte after it still belongs to the original text.
    println!("piece: {piece:?}, length {}", piece.len());
    println!("byte after it: {:?}", text.as_bytes()[piece.len()] as char);

    // A C function wants a NUL at the end, so it gets a copy that has one.
    let c_text = CString::new(piece).unwrap();
    println!("copy with NUL: {:?}", c_text.as_bytes_with_nul());

    // C's atoi has no way to say "too big"; parse does.
    println!("99999999999 as i32: {:?}", "99999999999".parse::<i32>());
}
