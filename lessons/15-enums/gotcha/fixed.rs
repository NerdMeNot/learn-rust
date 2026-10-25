enum Shape {
    Circle(f64),
    Rect(f64, f64),
    Triangle(f64, f64),
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(w, h) => w * h,
        Shape::Triangle(b, h) => 0.5 * b * h,
    }
}

fn main() {
    let shapes = [
        Shape::Circle(1.0),
        Shape::Rect(2.0, 3.0),
        Shape::Triangle(2.0, 3.0),
    ];
    for s in &shapes {
        println!("{}", area(s));
    }
}
