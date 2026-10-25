#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: u32,
    label: String,
}

fn main() {
    let c = Circle {
        center: Point { x: 1, y: -2 },
        radius: 5,
        label: String::from("home"),
    };
    println!("radius {}, label {}", c.radius, c.label);
    println!("center ({}, {})", c.center.x, c.center.y);
    println!("{{:?}}  -> {c:?}");
    println!("{{:#?}} -> {c:#?}");
}
