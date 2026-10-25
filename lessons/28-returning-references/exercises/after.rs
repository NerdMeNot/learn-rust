/// Return the part of `text` after the first `marker`, or "" when `marker` is not in `text`.
/// The result must be a slice of `text`, not a copy. The signature is part of the exercise:
/// say which input the result is borrowed from, so the caller can drop `marker` early.
fn after(text: &str, marker: &str) -> String {
    todo!("find the marker, slice text after it; change -> String so the result borrows from text")
}

/// True when `inner` is a slice of `outer`: it points inside the same bytes.
fn is_slice_of(outer: &str, inner: &str) -> bool {
    let start = outer.as_ptr() as usize;
    let end = start + outer.len();
    let at = inner.as_ptr() as usize;
    at >= start && at + inner.len() <= end
}

fn main() {
    let text = String::from("key: value: more");
    let rest;
    {
        let marker = String::from(": ");
        rest = after(&text, &marker);
    } // `marker` is dropped here; `rest` is still used below
    assert_eq!(rest, "value: more");
    assert!(
        is_slice_of(&text, &rest),
        "return a slice of text, not a copy"
    );

    // The marker is not there.
    assert_eq!(after("no colon", ": "), "");

    // Not English: the marker is three bytes long.
    assert_eq!(after("é→b", "→"), "b");

    println!("all checks passed");
}
