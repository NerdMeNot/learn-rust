struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 3, y: -2 };
    // like C++ `auto& [x, y] = p;`, but x and y are `&mut i32`, not the numbers
    let Point { x, y } = &mut p;
    x += 10;
    *y += 1;
    println!("{} {}", p.x, p.y);
}
