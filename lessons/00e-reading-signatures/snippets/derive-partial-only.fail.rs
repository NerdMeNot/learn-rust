#[derive(PartialOrd)]
struct Point {
    x: i32,
}

fn main() {
    let a = Point { x: 1 };
    let b = Point { x: 2 };
    println!("greater: {}", b > a);
}
