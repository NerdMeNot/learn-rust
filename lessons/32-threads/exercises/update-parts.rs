#![forbid(unsafe_code)]

use std::sync::Mutex;
use std::thread::{self, ThreadId};

// The checks use this to record which thread ran each part, and where the part starts in
// memory. Ignore it for now.
static SEEN: Mutex<Vec<(ThreadId, usize)>> = Mutex::new(Vec::new());

/// The work for one part: add the part's length to each element. It records its thread and
/// the part's address.
fn add_len(part: &mut [i32]) {
    SEEN.lock()
        .unwrap()
        .push((thread::current().id(), part.as_ptr() as usize));
    let n = part.len() as i32;
    for x in part.iter_mut() {
        *x += n;
    }
}

/// Cuts `data` into parts of `ceil(len / parts)` elements and calls `f` on each part on its
/// own thread. The parts are slices of `data` itself: nothing is copied, `data` changes in
/// place. `parts` is at least 1.
fn update_parts(data: &mut [i32], parts: usize, f: fn(&mut [i32])) {
    todo!(
        "two threads cannot hold &mut to the whole slice (E0499): cut it into pieces that do not overlap, and borrow one piece per thread"
    )
}

fn main() {
    let mut data = vec![0; 7];
    update_parts(&mut data, 3, add_len);
    assert_eq!(data, vec![3, 3, 3, 3, 3, 3, 1]); // parts of 3, 3 and 1
    {
        let seen = SEEN.lock().unwrap();
        let mut ids: Vec<_> = seen.iter().map(|(id, _)| format!("{id:?}")).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 3, "each part needs its own thread");
        assert!(
            !seen.iter().any(|(id, _)| *id == thread::current().id()),
            "not on main"
        );

        // changed in place: each part starts inside `data`'s own memory,
        // 3 elements (12 bytes) after the one before
        let start = data.as_ptr() as usize;
        let mut offsets: Vec<usize> = seen.iter().map(|(_, p)| p.wrapping_sub(start)).collect();
        offsets.sort();
        assert_eq!(
            offsets,
            vec![0, 12, 24],
            "the parts must be slices of `data`"
        );
    }

    let before = data.as_ptr();
    update_parts(&mut data, 2, |p| p.reverse());
    assert_eq!(data.as_ptr(), before);
    assert_eq!(
        data,
        vec![3, 3, 3, 3, 1, 3, 3],
        "each part reversed on its own"
    );

    let mut empty: Vec<i32> = Vec::new();
    update_parts(&mut empty, 4, add_len);
    assert!(empty.is_empty());

    println!("all checks passed");
}
