#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let mut b = a; // b holds its own copy of the two numbers
    b.x = 99;
    println!("a = ({}, {})", a.x, a.y);
    println!("b = ({}, {})", b.x, b.y);
}
