/// The first and the last non-empty piece of `text` between separators `sep`, or `None`
/// when there is no piece. Both pieces must be views into `text`: no copies.
///
/// The starter returns copies (`String`) so that it compiles. Change the signature so
/// the pieces borrow from `text`.
fn first_and_last(text: &str, sep: &str) -> Option<(String, String)> {
    todo!("text.split(sep), filter out empty pieces, next() and last(); then fix the types")
}

/// Do the bytes of `piece` lie inside the bytes of `text`? A copy would not.
fn inside(piece: &str, text: &str) -> bool {
    let start = text.as_ptr() as usize;
    let at = piece.as_ptr() as usize;
    at >= start && at + piece.len() <= start + text.len()
}

fn main() {
    let text = String::from("alpha,beta,,gamma,");
    // `sep` is gone before the pieces are used, so they must not borrow from it.
    let found;
    {
        let sep = String::from(",");
        found = first_and_last(&text, &sep);
    }
    let (first, last) = found.expect("there are pieces");
    assert_eq!(first, "alpha");
    assert_eq!(last, "gamma");
    assert!(inside(&*first, &text));
    assert!(inside(&*last, &text));

    // One piece is both the first and the last.
    let (first, last) = first_and_last("solo", ",").unwrap();
    assert_eq!(first, "solo");
    assert_eq!(last, "solo");

    // Leading, trailing and doubled separators make empty pieces; they do not count.
    let (first, last) = first_and_last(",a,,b,", ",").unwrap();
    assert_eq!(first, "a");
    assert_eq!(last, "b");

    // Nothing to return.
    assert_eq!(first_and_last("", ","), None);
    assert_eq!(first_and_last(",,,", ","), None);

    // Not English: a multi-byte separator and accented text.
    let text = String::from("zürich→bern→genève");
    let (first, last) = first_and_last(&text, "→").unwrap();
    assert_eq!(first, "zürich");
    assert_eq!(last, "genève");
    assert!(inside(&*last, &text));

    println!("all checks passed");
}
