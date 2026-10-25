//! Inner doc comment: describes the file (crate) it sits in.

/// Outer doc comment: documents the item below it.
fn add(a: i32, b: i32) -> i32 {
    // A line comment runs to the end of the line.
    /* A block comment /* can nest */ and still be one comment. */
    a + b
}

fn main() {
    println!("2 + 3 = {}", add(2, 3));
}
