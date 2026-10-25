// Backs the gotcha text: a `%d` with no argument is plain text and compiles. Never shown.
fn main() {
    let text = format!("count: %d");
    assert_eq!(text, "count: %d");
    println!("ok");
}
