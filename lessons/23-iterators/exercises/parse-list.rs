/// Parse "3, 4,5" into [3, 4, 5]. Spaces around each number are ignored.
/// If any piece is not a whole number, return Err("bad number: \"<piece>\"")
/// for the first such piece, with the piece trimmed.
fn parse_list(text: &str) -> Result<Vec<i32>, String> {
    todo!("split on ',', trim, parse, map_err, then collect into Result<Vec<_>, _>")
}

fn main() {
    assert_eq!(parse_list("3, 4,5"), Ok(vec![3, 4, 5]));
    assert_eq!(parse_list(" -4 , +5 "), Ok(vec![-4, 5]));
    // The first bad piece is the one reported.
    assert_eq!(parse_list("1, x, y"), Err("bad number: \"x\"".to_string()));
    assert_eq!(parse_list("1, 2.5"), Err("bad number: \"2.5\"".to_string()));
    // Empty input is an empty list, not an error.
    assert_eq!(parse_list(""), Ok(vec![]));
    assert_eq!(parse_list("   "), Ok(vec![]));
    println!("all checks passed");
}
