enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.0 * r * r,
        Shape::Rect(w, h) => w * h,
    }
}

fn main() {
    println!("{}", area(&Shape::Rect(2.0, 3.0)));
    println!("{}", area(&Shape::Circle(1.0)));
}
