use std::sync::atomic::{AtomicUsize, Ordering};

static CALLS: AtomicUsize = AtomicUsize::new(0);
static BAD_CALLS: AtomicUsize = AtomicUsize::new(0);

/// Swaps `v[i]` and `v[j]` without checking the indices.
///
/// # Safety
/// `i` and `j` must both be less than `v.len()`.
unsafe fn swap_unchecked(v: &mut [i32], i: usize, j: usize) {
    CALLS.fetch_add(1, Ordering::SeqCst);
    if i >= v.len() || j >= v.len() {
        // The real thing would be undefined behavior. For this exercise the mistake is
        // counted and nothing else happens, so a wrong answer cannot corrupt memory.
        BAD_CALLS.fetch_add(1, Ordering::SeqCst);
        return;
    }
    let p = v.as_mut_ptr();
    // SAFETY: both indices are in bounds, and `ptr::swap` allows `i == j`
    unsafe { std::ptr::swap(p.add(i), p.add(j)) }
}

#[derive(Debug, PartialEq)]
enum SwapError {
    OutOfRange { index: usize, len: usize },
}

/// A safe wrapper around `swap_unchecked`: check `i` first, then `j`, and call the unsafe
/// function only when both are in range. Say in a `// SAFETY:` comment why the call is sound.
fn swap_checked(v: &mut [i32], i: usize, j: usize) -> Result<(), SwapError> {
    for index in [i, j] {
        if index >= v.len() {
            return Err(SwapError::OutOfRange {
                index,
                len: v.len(),
            });
        }
    }
    // SAFETY: both indices were just checked to be less than `v.len()`
    unsafe { swap_unchecked(v, i, j) };
    Ok(())
}

fn main() {
    let mut v = [1, 2, 3];
    assert_eq!(swap_checked(&mut v, 0, 2), Ok(()));
    assert_eq!(v, [3, 2, 1]);
    assert_eq!(swap_checked(&mut v, 1, 1), Ok(()));
    assert_eq!(v, [3, 2, 1]);

    assert_eq!(
        swap_checked(&mut v, 7, 0),
        Err(SwapError::OutOfRange { index: 7, len: 3 })
    );
    assert_eq!(
        swap_checked(&mut v, 0, 9),
        Err(SwapError::OutOfRange { index: 9, len: 3 })
    );
    assert_eq!(
        swap_checked(&mut v, 7, 9),
        Err(SwapError::OutOfRange { index: 7, len: 3 })
    );
    assert_eq!(v, [3, 2, 1], "a rejected call must not change the slice");
    assert_eq!(
        swap_checked(&mut [], 0, 0),
        Err(SwapError::OutOfRange { index: 0, len: 0 })
    );

    // two calls were allowed to reach the unsafe function: no more, no fewer, and none wrong
    assert_eq!(
        CALLS.load(Ordering::SeqCst),
        2,
        "call the unsafe function once per valid swap, and only then"
    );
    assert_eq!(
        BAD_CALLS.load(Ordering::SeqCst),
        0,
        "the unsafe function was called with a bad index"
    );
    println!("all checks passed");
}
