// Backs the "bound" table in lesson.mdx. Never shown.
use std::collections::{BTreeSet, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::Add;

fn display<T: Display>(x: T) -> String {
    format!("{x}")
}
fn debug<T: Debug>(x: T) -> String {
    format!("{x:?}")
}
fn equal<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}
fn less<T: PartialOrd>(a: T, b: T) -> bool {
    a < b
}
fn sorted<T: Ord + Clone>(items: &[T]) -> Vec<T> {
    let mut v = items.to_vec();
    v.sort();
    v
}
fn ordered_set<T: Ord>(items: Vec<T>) -> usize {
    items.into_iter().collect::<BTreeSet<T>>().len()
}
fn count<T: Eq + Hash>(items: Vec<T>) -> usize {
    let mut map = HashMap::new();
    for item in items {
        *map.entry(item).or_insert(0) += 1;
    }
    map.len()
}
fn twice<T: Clone>(x: &T) -> (T, T) {
    (x.clone(), x.clone())
}
fn use_twice<T: Copy>(x: T) -> (T, T) {
    let y = x;
    (x, y)
}
fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
fn fresh<T: Default>() -> T {
    T::default()
}
fn widen<T: Into<u64>>(x: T) -> u64 {
    x.into()
}

fn main() {
    assert_eq!(display(1.5), "1.5");
    assert_eq!(debug("a"), "\"a\"");
    assert!(equal("a", "a") && !equal(1, 2));
    assert!(less(1.5, 2.5));
    assert_eq!(sorted(&[3, 1, 2]), [1, 2, 3]);
    assert_eq!(ordered_set(vec!["b", "a", "b"]), 2);
    assert_eq!(count(vec!["b", "a", "b"]), 2);
    assert_eq!(
        twice(&String::from("x")),
        ("x".to_string(), "x".to_string())
    );
    assert_eq!(use_twice(4), (4, 4));
    assert_eq!(add(1.5, 2.0), 3.5);
    assert_eq!(fresh::<Vec<i32>>(), Vec::<i32>::new());
    assert_eq!(widen(7u8), 7u64);
    assert_eq!(widen(7u32), 7u64);
    println!("ok");
}
