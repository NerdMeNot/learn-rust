enum Shape {
    Circle(f64),
    Rect(f64, f64),
    Triangle(f64, f64), // new!
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rect(w, h) => w * h,
    }
}

fn main() {
    println!("{}", area(&Shape::Triangle(2.0, 3.0)));
}
