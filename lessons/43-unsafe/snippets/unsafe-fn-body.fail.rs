#![deny(unsafe_op_in_unsafe_fn)]

/// # Safety
/// `p` must point to a valid, initialised `i32`.
unsafe fn read(p: *const i32) -> i32 {
    *p
}

fn main() {
    let x = 7;
    // SAFETY: `&raw const x` points at `x`, which is alive and initialised
    println!("{}", unsafe { read(&raw const x) });
}
