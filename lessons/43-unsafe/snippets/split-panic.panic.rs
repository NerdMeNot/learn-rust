use std::slice;

fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();
    assert!(mid <= len, "mid {mid} is past the end ({len})");
    // SAFETY: mid <= len, as asserted above, so both halves are in bounds and do not overlap
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = [1, 2, 3];
    let _halves = split_at_mut(&mut v, 9);
}
