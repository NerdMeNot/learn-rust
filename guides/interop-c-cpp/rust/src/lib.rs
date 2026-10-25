//! A small library for C and C++ programs to call.

use std::ffi::{CStr, CString, c_char};
use std::panic::catch_unwind;
use std::ptr;

/// Two doubles, in this order. C lays out `struct Point` the same way.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// Fields of different sizes, so that the layout rules matter.
#[repr(C)]
pub struct Reading {
    pub id: u8,
    pub value: f64,
    pub flag: u8,
}

/// An opaque handle: C only ever holds a pointer to it and never sees the fields.
pub struct Accumulator {
    total: i64,
}

#[unsafe(no_mangle)]
pub extern "C" fn point_distance(a: Point, b: Point) -> f64 {
    (a.x - b.x).hypot(a.y - b.y)
}

/// # Safety
/// `r` must point to a valid `Reading`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn reading_value(r: *const Reading) -> f64 {
    // SAFETY: the caller promised that `r` is valid (see above).
    unsafe { (*r).value }
}

#[unsafe(no_mangle)]
pub extern "C" fn accumulator_new() -> *mut Accumulator {
    Box::into_raw(Box::new(Accumulator { total: 0 }))
}

/// # Safety
/// `acc` must come from `accumulator_new` and must not have been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn accumulator_add(acc: &mut Accumulator, value: i64) -> i64 {
    acc.total += value;
    acc.total
}

/// # Safety
/// `acc` must be null, or come from `accumulator_new` and not have been freed.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn accumulator_free(acc: *mut Accumulator) {
    if !acc.is_null() {
        // SAFETY: made by `Box::into_raw` in `accumulator_new`, not freed yet.
        drop(unsafe { Box::from_raw(acc) });
    }
}

/// Adds up `len` numbers. A null pointer counts as an empty array.
///
/// # Safety
/// If `ptr` is not null, it must point to `len` readable `i32` values.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn sum_i32(ptr: *const i32, len: usize) -> i64 {
    if ptr.is_null() {
        return 0;
    }
    // SAFETY: not null, and the caller promised `len` readable values.
    let numbers = unsafe { std::slice::from_raw_parts(ptr, len) };
    numbers.iter().map(|&n| i64::from(n)).sum()
}

/// Returns a new string, "hello, <name>", that the caller must give back to
/// `greeting_free`. Returns null if `name` is null.
///
/// # Safety
/// If `name` is not null, it must point to a NUL-terminated string.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn greeting(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return ptr::null_mut();
    }
    // SAFETY: not null, and the caller promised a NUL-terminated string.
    let name = unsafe { CStr::from_ptr(name) }.to_string_lossy();
    match CString::new(format!("hello, {name}")) {
        Ok(text) => text.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// # Safety
/// `text` must be null, or a pointer returned by `greeting` that has not been freed yet.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn greeting_free(text: *mut c_char) {
    if !text.is_null() {
        // SAFETY: `greeting` made this pointer with `CString::into_raw`, and
        // the caller promised it is still unfreed.
        drop(unsafe { CString::from_raw(text) });
    }
}

/// Panics when `fail` is not zero. A panic cannot leave an `extern "C"` function.
#[unsafe(no_mangle)]
pub extern "C" fn explode(fail: i32) -> i32 {
    if fail != 0 {
        panic!("boom");
    }
    7
}

/// Divides `a` by `b` and writes the answer to `*out`. Returns 0 on success and
/// -1 if the division panicked (a zero `b`).
///
/// # Safety
/// `out` must be valid for writing one `i32`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn checked_div(a: i32, b: i32, out: *mut i32) -> i32 {
    match catch_unwind(|| a / b) {
        Ok(q) => {
            // SAFETY: the caller promised that `out` can be written.
            unsafe { *out = q };
            0
        }
        Err(_) => -1,
    }
}

/// Like `explode`, but declared so that the panic may unwind into the caller.
#[unsafe(no_mangle)]
pub extern "C-unwind" fn explode_unwind() {
    panic!("boom, unwinding");
}
