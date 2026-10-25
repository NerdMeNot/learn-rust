struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2 };
    p.xx = 10;
    println!("{} {}", p.x, p.y);
}
