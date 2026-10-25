fn f(a: &str, n: usize) -> &str {
    &a[..n]
}

fn main() {
    assert_eq!(f("xyz", 2), "xy");
}
