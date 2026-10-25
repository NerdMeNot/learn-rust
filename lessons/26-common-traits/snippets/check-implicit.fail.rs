// Rust never converts implicitly: a &str is not a String. Not shown on the page.
fn greet(name: String) -> usize {
    name.len()
}

fn main() {
    println!("{}", greet("ana"));
}
