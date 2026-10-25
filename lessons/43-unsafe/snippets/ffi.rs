use std::ffi::{CString, c_char};

unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
    safe fn sqrt(x: f64) -> f64;
}

fn main() {
    let text = CString::new("héllo").unwrap();
    // SAFETY: `text` is a valid NUL-terminated string that lives until after the call
    let n = unsafe { strlen(text.as_ptr()) };
    println!("strlen(\"héllo\") = {n} bytes");
    println!("sqrt(81) = {}", sqrt(81.0));
    println!("str::len = {}", "héllo".len());
}
