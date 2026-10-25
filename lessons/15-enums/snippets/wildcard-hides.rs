enum Shape {
    Circle(f64),
    Rect(f64, f64),
    Triangle(f64, f64), // added later
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(w, h) => w * h,
        _ => 0.0, // "everything else": this now includes Triangle
    }
}

// no wildcard: adding Triangle made the compiler send us here
fn perimeter(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 2.0 * 3.14 * r,
        Shape::Rect(w, h) => 2.0 * (w + h),
        Shape::Triangle(a, b) => a + b + (a * a + b * b).sqrt(), // right triangle
    }
}

fn main() {
    let shapes = [
        Shape::Circle(1.0),
        Shape::Rect(2.0, 3.0),
        Shape::Triangle(3.0, 4.0),
    ];
    for s in &shapes {
        println!("area: {}, perimeter: {}", area(s), perimeter(s));
    }
}
