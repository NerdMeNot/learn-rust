#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a.clone();
    println!("{{:?}}  -> {:?}", a);
    println!("{{:#?}} -> {:#?}", a);
    println!("equal: {}", a == b);
}
