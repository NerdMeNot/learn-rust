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

fn circle_area(c: Shape::Circle) -> f64 {
    area(&c)
}

fn main() {
    println!("{}", area(&Shape::Rect(2.0, 3.0)));
}
