enum Shape {
    Circle { r: f64 },
    Square { side: f64 },
    Triangle,
}

fn describe(s: &Shape) {
    match s {
        Shape::Circle { r } => println!("circle {}", 3.0 * r * r),
        Shape::Square { side } => println!("square {}", side * side),
        _ => {}
    }
}

fn main() {
    describe(&Shape::Circle { r: 1.0 });
    describe(&Shape::Square { side: 2.0 });
    describe(&Shape::Triangle);
    println!("done");
}
