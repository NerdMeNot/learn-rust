// C: while ((c = *p++) != '\0') { ... }   Rust: take the next item, and stop when there is none.
fn main() {
    let mut chars = "abc".chars();
    while let Some(c) = chars.next() {
        println!("char: {c}");
    }
}
