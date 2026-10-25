/// Append `parts` to `out`, with `sep` between them (not before the first or after the last).
/// What `out` already holds stays.
fn join_into(parts: &[&str], sep: &str, out: &mut String) {
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            out.push_str(sep);
        }
        out.push_str(part);
    }
}

fn main() {
    let mut out = String::with_capacity(32);
    out.push('>');
    let buffer = out.as_ptr();

    join_into(&["a", "b", "c"], ", ", &mut out);
    assert_eq!(out, ">a, b, c");
    assert_eq!(
        out.as_ptr(),
        buffer,
        "write into the caller's String, don't replace it"
    );
    assert_eq!(out.capacity(), 32);

    // One part: no separator. No parts: nothing added.
    out.clear();
    join_into(&["solo"], ", ", &mut out);
    assert_eq!(out, "solo");
    join_into(&[], ", ", &mut out);
    assert_eq!(out, "solo");

    // Not English, and a separator that is more than one byte.
    out.clear();
    join_into(&["é", "ü"], "·", &mut out);
    assert_eq!(out, "é·ü");
    assert_eq!(out.as_ptr(), buffer);

    println!("all checks passed");
}
