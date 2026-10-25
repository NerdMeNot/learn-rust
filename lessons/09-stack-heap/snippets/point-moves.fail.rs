// The same Point as above, but without #[derive(Clone, Copy)].
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;
    println!("b = ({}, {})", b.x, b.y);
    println!("a = ({}, {})", a.x, a.y); // a was moved into b
}
