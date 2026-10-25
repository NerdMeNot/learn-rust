#![forbid(unsafe_code)]

use std::sync::Mutex;
use std::thread::{self, ThreadId};

// The checks use this to record which thread ran each chunk, and where the chunk starts in
// memory. Ignore it for now.
static SEEN: Mutex<Vec<(ThreadId, usize)>> = Mutex::new(Vec::new());

/// The work for one chunk. It records which thread ran it and the chunk's address.
fn total(chunk: &[u64]) -> u64 {
    SEEN.lock()
        .unwrap()
        .push((thread::current().id(), chunk.as_ptr() as usize));
    chunk.iter().sum()
}

/// Cuts `data` into chunks of `ceil(len / parts)` elements (so at most `parts` chunks, and
/// none when `data` is empty), calls `f` on each chunk on its own thread, and gives back the
/// results in chunk order. `data` is borrowed, never copied. `parts` is at least 1.
/// `f` is a plain function (`fn` pointer): it cannot capture anything, so any thread may call it.
fn chunk_results(data: &[u64], parts: usize, f: fn(&[u64]) -> u64) -> Vec<u64> {
    todo!(
        "chunks() panics on a chunk size of 0, so handle empty data first; the threads must borrow their chunks from `data`, not copy them"
    )
}

fn main() {
    let data: Vec<u64> = (1..=10).collect();

    assert_eq!(chunk_results(&data, 3, total), vec![10, 26, 19]); // [1..=4] [5..=8] [9, 10]
    {
        let seen = SEEN.lock().unwrap();
        let mut ids: Vec<_> = seen.iter().map(|(id, _)| format!("{id:?}")).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), 3, "each chunk needs its own thread");
        assert!(
            !seen.iter().any(|(id, _)| *id == thread::current().id()),
            "not on main"
        );

        // borrowed, not copied: each chunk starts inside `data`'s own memory,
        // 4 elements (32 bytes) after the one before
        let start = data.as_ptr() as usize;
        let mut offsets: Vec<usize> = seen.iter().map(|(_, p)| p.wrapping_sub(start)).collect();
        offsets.sort();
        assert_eq!(
            offsets,
            vec![0, 32, 64],
            "the chunks must be slices of `data`"
        );
    }

    assert_eq!(chunk_results(&data, 1, total), vec![55]);
    assert_eq!(
        chunk_results(&data, 20, total),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    assert_eq!(chunk_results(&[], 4, total), Vec::<u64>::new());

    // the data is still ours afterwards, and the order of results is the order of chunks
    assert_eq!(data.len(), 10);
    let firsts = chunk_results(&data, 5, |c| c[0]);
    assert_eq!(firsts, vec![1, 3, 5, 7, 9]);

    println!("all checks passed");
}
