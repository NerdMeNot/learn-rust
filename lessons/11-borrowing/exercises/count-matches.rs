/// How many items in `xs` equal `target`? Both are borrowed: the numbers are not copied
/// into the function and nothing is used up.
fn count_matches(xs: &[i32], target: &i32) -> usize {
    todo!("for x in xs gives &i32 items; compare them with target, or compare *x with *target")
}

fn main() {
    let list = vec![4, 7, 4, 9, 4];

    // A reference to a literal.
    assert_eq!(count_matches(&list, &4), 3);
    assert_eq!(count_matches(&list, &5), 0);

    // A reference into the same Vec we are searching: two borrows alive at once.
    assert_eq!(count_matches(&list, &list[1]), 1);
    assert_eq!(count_matches(&list, &list[0]), 3);

    // Part of the Vec, and an array.
    assert_eq!(count_matches(&list[1..], &4), 2);
    assert_eq!(count_matches(&[1, 1, 2], &1), 2);

    // Nothing to search.
    assert_eq!(count_matches(&[], &1), 0);

    // The Vec was only borrowed.
    assert_eq!(list.len(), 5);

    println!("all checks passed");
}
