//! Prints a few values in different formats.

/// A point on a grid. `derive(Debug)` lets `{:?}` print it.
#[derive(Debug)]
#[allow(dead_code)] // the fields are only printed, never read
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 }; // a line comment
    let items = vec![1, 2, 3];
    assert_eq!(items.len(), 3); // stops with a message if false
    println!("point: {p:?}");
    println!("items: {items:?}");
    println!("row: [{:>5}] [{:<5}] [{:05}]", 42, 42, 42);
    let pi = format!("{:.2}", 3.14159);
    println!("pi: {pi} {{done}}");
}
