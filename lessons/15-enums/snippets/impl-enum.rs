enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 3.14 * radius * radius,
            Shape::Rect { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rect { .. } => "rectangle",
            Shape::Triangle { .. } => "triangle",
        }
    }

    // an "associated function", called as Shape::square(2.0)
    fn square(side: f64) -> Shape {
        Shape::Rect {
            width: side,
            height: side,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle { radius: 1.0 },
        Shape::square(2.0),
        Shape::Triangle {
            base: 3.0,
            height: 4.0,
        },
    ];
    for s in &shapes {
        println!("{}: area {}", s.name(), s.area());
    }
}
