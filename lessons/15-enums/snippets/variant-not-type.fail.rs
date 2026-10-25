enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

// Circle is a variant of Shape, not a type of its own
fn radius(c: Shape::Circle) -> f64 {
    c.0
}

fn main() {
    let _ = Shape::Rect(2.0, 3.0);
    println!("{}", radius(Shape::Circle(1.0)));
}
