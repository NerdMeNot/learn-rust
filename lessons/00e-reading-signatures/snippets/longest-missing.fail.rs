fn longest(x: &str, y: &str) -> &str {
    if y.len() > x.len() { y } else { x }
}

fn main() {
    println!("longest: {}", longest("ab", "abc"));
}
