// Backs the table in the lesson: every "call -> result" cell is asserted here.
fn main() {
    let some = Some(4);
    let none: Option<i32> = None;
    let words = Some("hi");

    assert_eq!(some.map(|n| n * 2), Some(8));
    assert_eq!(none.map(|n| n * 2), None);
    assert_eq!(
        some.and_then(|n| if n > 3 { Some(n * 10) } else { None }),
        Some(40)
    );
    assert_eq!(
        some.and_then(|n| if n > 5 { Some(n * 10) } else { None }),
        None
    );
    assert_eq!(some.filter(|n| n % 2 == 1), None);
    assert_eq!(some.filter(|n| n % 2 == 0), Some(4));
    assert_eq!(some.zip(words), Some((4, "hi")));
    assert_eq!(none.zip(words), None);
    assert_eq!(some.unwrap_or(0), 4);
    assert_eq!(Some(0).unwrap_or(5), 0);
    assert_eq!(none.unwrap_or(0), 0);
    assert_eq!(none.unwrap_or_else(|| 1 + 1), 2);
    assert_eq!(none.unwrap_or_default(), 0);
    assert_eq!(some.ok_or("missing"), Ok(4));
    assert_eq!(none.ok_or("missing"), Err("missing"));
    assert_eq!(none.or(Some(9)), Some(9));
    assert_eq!(some.or(Some(9)), Some(4));
    assert_eq!(some.xor(none), Some(4));
    assert_eq!(some.xor(Some(1)), None);
    assert!(some.is_some() && !some.is_none());
    assert!(some.is_some_and(|n| n > 3));
    assert!(!none.is_some_and(|n| n > 3));
    assert!(none.is_none_or(|n| n > 3));
    assert_eq!(Some(Some(3)).flatten(), Some(3));
    assert_eq!(some.map_or(0, |n| n + 1), 5);
    assert_eq!(none.map_or(0, |n| n + 1), 0);
    assert_eq!(some.iter().count(), 1);
    assert!(none < some);
    assert!(Some(1) < Some(2));
    println!("all table cells hold");
}
