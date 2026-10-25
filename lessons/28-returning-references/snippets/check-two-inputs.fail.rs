fn f(a: &str, b: &str) -> &str {
    if a.is_empty() { b } else { a }
}

fn main() {
    println!("{}", f("x", "y"));
}
