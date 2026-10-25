// The signature is almost right. Fill in the body, and add what `T` needs.
fn largest<T>(list: &[T]) -> Option<&T> {
    todo!(
        "return None early if the list is empty; otherwise keep a `best` reference and compare each item; the compiler will say which bound T needs"
    )
}

fn main() {
    assert_eq!(largest(&[3, 9, 4]), Some(&9));
    assert_eq!(largest(&[1.5, -2.0]), Some(&1.5));
    assert_eq!(largest(&["pear", "fig"]), Some(&"pear"));
    let empty: [i32; 0] = [];
    assert_eq!(largest(&empty), None);
    println!("all checks passed");
}
