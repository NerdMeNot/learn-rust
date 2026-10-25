/// Build the text of a one-entry JSON object, such as `{"age": 36}`.
/// Hint: in a format string, `{{` prints one `{` and `}}` prints one `}`.
fn json_pair(key: &str, value: i32) -> String {
    format!("{{\"{key}\": {value}}}")
}

fn main() {
    assert_eq!(json_pair("age", 36), "{\"age\": 36}");
    assert_eq!(json_pair("score", -5), "{\"score\": -5}");
    // The case to watch: braces inside an argument are printed as they are.
    assert_eq!(json_pair("{x}", 1), "{\"{x}\": 1}");
    println!("all checks passed");
}
