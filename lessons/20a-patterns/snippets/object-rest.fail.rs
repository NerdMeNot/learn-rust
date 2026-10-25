struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point { x: 1, y: 2, z: 3 };
    // `..` skips fields; it cannot take a name like JavaScript's `...rest`
    let Point { x, ..rest } = p;
    println!("x = {x}");
}
