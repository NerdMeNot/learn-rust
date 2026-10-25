// `Copy` is a label that makes `let b = a` copy the bytes. More on it below.
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let mut b = a;
    b.x = 99;

    println!("a: x = {}, y = {}", a.x, a.y);
    println!("b: x = {}, y = {}", b.x, b.y);
    println!("size of a: {} bytes", size_of_val(&a));
    println!("size of b: {} bytes", size_of_val(&b));
}
