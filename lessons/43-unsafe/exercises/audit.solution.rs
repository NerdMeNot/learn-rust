// `blank` makes rustc warn (invalid_value); the lint is allowed so the file stays quiet
#![allow(dead_code, invalid_value)]

use std::slice;

/// What a `// SAFETY:` comment on the unsafe block below would have to promise,
/// and the code does not make true.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Missing {
    /// Nothing is missing: the code establishes everything its unsafe block needs.
    NothingMissing,
    /// The index is within the slice.
    InBounds,
    /// The pointer has the alignment of the type it is read as.
    Aligned,
    /// The pointer points at something that is still alive.
    Alive,
    /// The memory holds a valid value of the type.
    Initialized,
    /// No other reference to the same memory exists while this one is used.
    NotAliased,
}

fn first(v: &[u32]) -> u32 {
    // SAFETY: `v` is a valid slice, so its pointer is valid for reads
    unsafe { *v.as_ptr() }
}

fn as_words(bytes: &[u8]) -> &[u32] {
    // SAFETY: the pointer is valid for `bytes.len() / 4` words of 4 bytes each
    unsafe { slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4) }
}

fn leftover() -> i32 {
    let p = {
        let x = 7;
        &raw const x
    };
    // SAFETY: `p` was made from a valid variable
    unsafe { *p }
}

fn blank() -> String {
    // SAFETY: a `String` is just bytes, so all zero bytes are a valid `String`
    unsafe { std::mem::zeroed() }
}

fn last(v: &[u32]) -> Option<u32> {
    if v.is_empty() {
        return None;
    }
    // SAFETY: `v` is not empty, so `len - 1` is in bounds, and the pointer comes from a live slice
    Some(unsafe { *v.as_ptr().add(v.len() - 1) })
}

fn twice(v: &mut [i32]) -> (&mut i32, &mut i32) {
    assert!(!v.is_empty());
    let p = v.as_mut_ptr();
    // SAFETY: `p` is valid for reads and writes of one `i32`, since `v` is not empty
    unsafe { (&mut *p, &mut *p) }
}

/// For each function above, in order (`first`, `as_words`, `leftover`, `blank`, `last`,
/// `twice`), say which condition its unsafe block relies on without making it true.
/// Do not call them: five of the six are undefined behavior on some input.
fn audit() -> [Missing; 6] {
    [
        Missing::InBounds,
        Missing::Aligned,
        Missing::Alive,
        Missing::Initialized,
        Missing::NothingMissing,
        Missing::NotAliased,
    ]
}

fn main() {
    let expected = [
        Missing::InBounds,
        Missing::Aligned,
        Missing::Alive,
        Missing::Initialized,
        Missing::NothingMissing,
        Missing::NotAliased,
    ];
    let names = ["first", "as_words", "leftover", "blank", "last", "twice"];
    let hints = [
        "what does `v.as_ptr()` point at when `v` has no elements?",
        "a `&[u8]` can start at any address; which addresses may a `u32` live at?",
        "where does `x` live, and when does that end?",
        "can all-zero bytes be a `String`? Think about its pointer field.",
        "read the code above the block: is each condition already made true there?",
        "how many `&mut` to the same `i32` exist at once? (the `assert!` already handles the empty slice)",
    ];
    let guess = audit();
    // the first wrong answer is named, with a question to reason from, not the answer
    for (i, (g, e)) in guess.iter().zip(&expected).enumerate() {
        assert!(
            g == e,
            "{}: you answered {g:?}. Ask: {}",
            names[i],
            hints[i]
        );
    }
    // the one sound function really is sound: it handles both of the cases that matter
    assert_eq!(last(&[]), None);
    assert_eq!(last(&[1, 2, 3]), Some(3));
    println!("all checks passed");
}
