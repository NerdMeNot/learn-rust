/// Turn "3-7" into Ok((3, 7)). Spaces around either number are fine.
/// (Hint: ok_or("message")? works because a &str converts into a String error.)
/// Return Err with a short message when:
///   - there is no '-'                      ("missing '-'")
///   - either side is not a whole number    (a message starting "bad start" or "bad end")
///   - the start is bigger than the end     ("start is after end")
fn parse_range(text: &str) -> Result<(u32, u32), String> {
    let (left, right) = text.split_once('-').ok_or("missing '-'")?;
    let start: u32 = left
        .trim()
        .parse()
        .map_err(|e| format!("bad start {left:?}: {e}"))?;
    let end: u32 = right
        .trim()
        .parse()
        .map_err(|e| format!("bad end {right:?}: {e}"))?;
    if start > end {
        return Err("start is after end".to_string());
    }
    Ok((start, end))
}

fn main() {
    assert_eq!(parse_range("3-7"), Ok((3, 7)));
    assert_eq!(parse_range(" 3 - 7 "), Ok((3, 7)));
    assert_eq!(parse_range("5-5"), Ok((5, 5)));
    assert_eq!(parse_range("7"), Err("missing '-'".to_string()));
    assert_eq!(parse_range("7-3"), Err("start is after end".to_string()));
    assert_eq!(parse_range(""), Err("missing '-'".to_string()));
    // "-7" has an empty start, and "3-4-5" has "4-5" as the end. The Arabic-Indic
    // digits in the last one are numbers to people, but not to parse.
    for bad in ["3-", "-7", "a-b", "3-4-5", "٣-٥"] {
        let message = parse_range(bad).unwrap_err();
        assert!(message.starts_with("bad "), "{bad:?} gave {message:?}");
    }
    println!("all checks passed");
}
