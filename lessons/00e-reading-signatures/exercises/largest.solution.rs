// The signature is almost right. Fill in the body, and add what `T` needs.
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    let mut best = &list[0];
    for item in list {
        if item > best {
            best = item;
        }
    }
    Some(best)
}

fn main() {
    assert_eq!(largest(&[3, 9, 4]), Some(&9));
    assert_eq!(largest(&[1.5, -2.0]), Some(&1.5));
    assert_eq!(largest(&["pear", "fig"]), Some(&"pear"));
    let empty: [i32; 0] = [];
    assert_eq!(largest(&empty), None);
    println!("all checks passed");
}
