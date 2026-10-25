#![allow(non_camel_case_types)]

mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use std::ffi::CStr;

/// A safe wrapper: the only place that calls into C.
fn compute(values: &[f64]) -> Result<ffi::stats_t, String> {
    let mut out = ffi::stats_t { count: 0, sum: 0.0, max: 0.0 };
    // SAFETY: `values` is valid for `values.len()` reads, and `out` for one write.
    let status = unsafe { ffi::stats_compute(values.as_ptr(), values.len(), &mut out) };
    if status == 0 {
        Ok(out)
    } else {
        // SAFETY: C returns a pointer to a NUL-terminated string literal that lives
        // as long as the program, and we only borrow it.
        let why = unsafe { CStr::from_ptr(ffi::stats_describe_status(status)) };
        Err(why.to_string_lossy().into_owned())
    }
}

fn main() {
    match compute(&[2.0, 4.5, 3.5]) {
        Ok(s) => println!("count {}, sum {}, max {}", s.count, s.sum, s.max),
        Err(e) => println!("error: {e}"),
    }
    match compute(&[]) {
        Ok(_) => println!("unexpected"),
        Err(e) => println!("error: {e}"),
    }
}
