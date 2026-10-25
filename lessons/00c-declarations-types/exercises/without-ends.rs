/// Everything except the first and last item. A slice with fewer than 2 items has none left.
fn without_ends(items: &[i32]) -> &[i32] {
    todo!("a range like `1..items.len() - 1` builds the slice; check the length first")
}

fn main() {
    assert_eq!(without_ends(&[1, 2, 3, 4]), [2, 3]);
    assert_eq!(without_ends(&[1, 2, 3]), [2]);
    assert_eq!(without_ends(&[1, 2]), []);
    assert_eq!(without_ends(&[1]), []);
    // The case to watch: `0 - 1` on an unsigned length panics.
    assert_eq!(without_ends(&[]), []);
    println!("all checks passed");
}
