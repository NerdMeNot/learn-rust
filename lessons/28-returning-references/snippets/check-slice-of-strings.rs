fn f(words: &[String]) -> &str {
    &words[0]
}

fn main() {
    assert_eq!(f(&[String::from("x")]), "x");
}
