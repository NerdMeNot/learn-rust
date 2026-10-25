enum Shape {
    Circle(f64),    // radius
    Rect(f64, f64), // width, height
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(w, h) => w * h,
    }
}

fn main() {
    let shapes = [Shape::Circle(1.0), Shape::Rect(2.0, 3.0)];
    for s in &shapes {
        println!("{}", area(s));
    }
}
