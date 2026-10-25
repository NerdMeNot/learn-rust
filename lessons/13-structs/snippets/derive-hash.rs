use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut seen = HashSet::new();
    seen.insert(Point { x: 1, y: 2 });
    seen.insert(Point { x: 1, y: 2 });
    seen.insert(Point { x: 3, y: 4 });
    println!("distinct points: {}", seen.len());
    println!("has (3, 4): {}", seen.contains(&Point { x: 3, y: 4 }));
}
