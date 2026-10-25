// Backs the "in Rust" cells of the tables in lesson.mdx. Never shown.
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::ops::Add;

// Go table.
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn push(&mut self, x: T) {
        self.items.push(x);
    }

    fn map<U>(&self, f: impl Fn(&T) -> U) -> Vec<U> {
        self.items.iter().map(f).collect()
    }
}

fn map_all<T, U>(s: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    s.iter().map(f).collect()
}

fn same<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}

fn key<T: Eq + std::hash::Hash>(x: T) -> usize {
    let mut set = HashSet::new();
    set.insert(x);
    let mut map = HashMap::new();
    map.insert(1, 2);
    set.len() + map.len()
}

fn double<T: Add<Output = T> + Copy>(x: T) -> T {
    x + x
}

// Java, Kotlin, C# table.
fn biggest<T: PartialOrd>(a: T, b: T) -> T {
    if b > a { b } else { a }
}

fn both<T: Display + Clone>(x: &T) -> String {
    format!("{x}{}", x.clone())
}

fn fresh<T: Default>() -> T {
    T::default()
}

fn describe<T: Display + PartialEq + std::hash::Hash>(x: &T) -> String {
    format!("{x} {}", x == x)
}

struct Pair<T>(T, T);

impl<T> Pair<T> {
    fn map<U>(self, f: impl Fn(T) -> U) -> Pair<U> {
        Pair(f(self.0), f(self.1))
    }
}

// Python and TypeScript table.
trait HasLength {
    fn length(&self) -> usize;
}

impl HasLength for String {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

enum Item {
    Number(i32),
    Text(&'static str),
}

fn longer<T: HasLength>(a: T, b: T) -> T {
    if b.length() > a.length() { b } else { a }
}

// C and C++ table.
fn show_it(x: impl PartialOrd) -> bool {
    x < x
}

fn sum_array<T: Copy + Default + Add<Output = T>, const N: usize>(a: [T; N]) -> T {
    let mut total = T::default();
    for x in a {
        total = total + x;
    }
    total
}

fn main() {
    let mut stack = Stack { items: Vec::new() };
    stack.push(4);
    assert_eq!(stack.map(|n| n * 2), [8]);
    assert_eq!(
        map_all::<i32, String>(&[1, 2], |n| (n * 2).to_string()),
        ["2", "4"]
    );
    assert!(same("a", "a"));
    assert_eq!(key("a"), 2);
    assert_eq!(double(2.5), 5.0);
    assert_eq!(double(21), 42);

    assert_eq!(biggest("kiwi", "pear"), "pear");
    assert_eq!(both(&7), "77");
    assert_eq!(fresh::<String>(), "");
    assert_eq!(describe(&7), "7 true");
    assert_eq!(Pair(1, 2).map(|n| format!("#{n}")).0, "#1");
    assert!(fresh::<Vec<String>>().is_empty());

    assert_eq!(longer(String::from("ab"), String::from("abc")), "abc");
    assert_eq!(longer(vec![1], vec![1, 2]), [1, 2]);

    let mixed = vec![Item::Number(1), Item::Text("a")];
    assert_eq!(mixed.len(), 2);
    assert!(matches!(mixed[0], Item::Number(1)) && matches!(mixed[1], Item::Text("a")));
    show_it(1.5);

    assert_eq!(sum_array([1, 2, 3]), 6);
    assert_eq!(biggest::<f64>(1.0, 2.5), 2.5);
    println!("ok");
}
