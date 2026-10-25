fn f(a: &str) -> &str {
    a
}

fn main() {
    assert_eq!(f("x"), "x");
}
