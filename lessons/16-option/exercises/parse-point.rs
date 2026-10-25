/// Turn "3,4" into Some((3, 4)). Anything that is not two whole numbers
/// separated by a comma gives None.
fn parse_point(text: &str) -> Option<(i32, i32)> {
    todo!("split_once(',')? then parse each side: .parse().ok()?")
}

fn main() {
    assert_eq!(parse_point("3,4"), Some((3, 4)));
    assert_eq!(parse_point("-3,40"), Some((-3, 40)));
    // No comma.
    assert_eq!(parse_point("34"), None);
    // Empty input.
    assert_eq!(parse_point(""), None);
    // A side that is not a number, or is missing.
    assert_eq!(parse_point("3,x"), None);
    assert_eq!(parse_point("x,4"), None);
    assert_eq!(parse_point("3,"), None);
    // Three parts: "4,5" is not a number.
    assert_eq!(parse_point("3,4,5"), None);

    println!("all checks passed");
}
