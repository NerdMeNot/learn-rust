enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn describe(s: &Shape) -> String {
    match s {
        // a guard: the arm applies only when the condition is true
        Shape::Circle { radius } if *radius == 0.0 => String::from("a dot"),
        Shape::Circle { radius } => format!("a circle, radius {radius}"),
        // a pattern can test a field against a value
        Shape::Rect { width, height: 1.0 } => format!("a strip, {width} wide"),
        Shape::Rect { width, height } if width == height => format!("a square, side {width}"),
        // `..` means "I don't care about the other fields"
        Shape::Rect { width, .. } => format!("a rectangle, {width} wide"),
        Shape::Triangle { base, height } => format!("a triangle, {base} by {height}"),
    }
}

fn main() {
    let shapes = [
        Shape::Circle { radius: 0.0 },
        Shape::Circle { radius: 2.5 },
        Shape::Rect {
            width: 7.0,
            height: 1.0,
        },
        Shape::Rect {
            width: 3.0,
            height: 3.0,
        },
        Shape::Rect {
            width: 3.0,
            height: 4.0,
        },
        Shape::Triangle {
            base: 1.0,
            height: 2.0,
        },
    ];
    for s in &shapes {
        println!("{}", describe(s));
    }
}
