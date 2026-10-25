fn main() {
    let tab = "a\tb"; // escapes work in normal strings
    let raw = r"C:\temp\new"; // raw: backslashes stay
    let quoted = r#"she said "hi""#; // raw with quotes inside: add # on both ends
    let long = "one \
                two"; // a backslash at the end of a line skips the newline and the indent
    let lines = "first
second"; // a normal string may span lines

    println!("tab: {tab}");
    println!("raw: {raw} ({} bytes)", raw.len());
    println!("quoted: {quoted}");
    println!("long: {long}");
    println!("lines: {lines:?}");
    println!("crab: {} {}", '\u{1F980}', "\u{e9}");
}
