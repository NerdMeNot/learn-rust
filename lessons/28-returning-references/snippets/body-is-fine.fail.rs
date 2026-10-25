fn first_of(a: &str, b: &str) -> &str {
    println!("ignoring {b:?}");
    a
}

fn main() {
    println!("{}", first_of("ab", "abc"));
}
