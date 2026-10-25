struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point { x: 1, y: 2, z: 3 };
    // a struct pattern must mention every field, or end in `..`
    let Point { x } = p;
    println!("x = {x}");
}
