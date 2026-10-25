/// "3, 2.5" -> Some((3, 2.5)). Anything that is not "integer, number" -> None.
fn int_float(text: &str) -> Option<(i32, f64)> {
    todo!(
        "split_once(',') gives Some((left, right)); parse::<i32>() gives Ok(n) or Err(_), so try `let Ok(n) = ... else` with `return None`; trim() drops spaces"
    )
}

fn main() {
    assert_eq!(int_float("3,2.5"), Some((3, 2.5)));
    assert_eq!(int_float(" 7 , 0.5 "), Some((7, 0.5)));
    assert_eq!(int_float("3,4"), Some((3, 4.0)));
    assert_eq!(int_float("x,1.0"), None);
    assert_eq!(int_float("3"), None);
    assert_eq!(int_float("3,abc"), None);
    // The case to watch: the first part must be a whole number.
    assert_eq!(int_float("2.5,3"), None);
    println!("all checks passed");
}
