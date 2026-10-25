fn f(words: &[&str]) -> &str {
    words[0]
}

fn main() {
    println!("{}", f(&["x"]));
}
