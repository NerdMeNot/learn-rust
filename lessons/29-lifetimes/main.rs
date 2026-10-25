// 'a says: the result borrows from both inputs, so it is valid only while both are.
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// A struct that holds a reference says how long the borrowed text must live.
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap_or("");
    let excerpt = Excerpt { part: first };
    let l = longest(excerpt.part, "short");
    println!("longest: {l}");
    println!("excerpt length: {}", excerpt.part.len());
}
