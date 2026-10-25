use std::slice;

/// Splits `v` into two non-overlapping mutable halves at `mid`.
fn split_at_mut(v: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = v.len();
    let ptr = v.as_mut_ptr();
    assert!(mid <= len, "mid {mid} is past the end ({len})");
    // SAFETY: `ptr` is valid for `len` elements and we hold the only `&mut` to them.
    // mid <= len, so [0, mid) and [mid, len) are in bounds and do not overlap.
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut v = [1, 2, 3, 4, 5];
    let (a, b) = split_at_mut(&mut v, 2);
    a[0] = 10;
    b[0] = 30;
    println!("halves: {a:?} and {b:?}");
    println!("whole: {v:?}");
}
