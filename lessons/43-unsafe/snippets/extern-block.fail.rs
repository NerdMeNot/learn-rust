use std::ffi::{CString, c_char};
extern "C" {
    fn strlen(s: *const c_char) -> usize;
}
fn main() {
    let s = CString::new("hello").unwrap();
    println!("{}", unsafe { strlen(s.as_ptr()) });
}
