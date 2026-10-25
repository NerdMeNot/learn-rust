struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    center: Point,
    radius: u32,
}

fn main() {
    let mut c = Circle {
        center: Point { x: 1, y: -2 },
        radius: 5,
    };
    println!("x: {}", c.center.x);
    c.center.x += 10;
    c.radius = 8;
    println!(
        "center: ({}, {}), radius: {}",
        c.center.x, c.center.y, c.radius
    );
    println!("size of Point: {} bytes", size_of::<Point>());
    println!("size of Circle: {} bytes", size_of::<Circle>());
}
