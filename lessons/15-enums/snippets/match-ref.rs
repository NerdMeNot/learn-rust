enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

fn describe(s: &Shape) -> String {
    match s {
        // s is a reference, so the fields are references too: r is a &f64
        Shape::Circle(r) => {
            let radius: f64 = *r; // `*` follows the reference to the number
            format!("circle, radius {radius}")
        }
        Shape::Rect(w, h) => {
            // arithmetic and comparison between two references work as they are
            let kind = if w == h { "square" } else { "rectangle" };
            format!("{kind}, area {}", w * h)
        }
    }
}

fn main() {
    println!("{}", describe(&Shape::Circle(1.5)));
    println!("{}", describe(&Shape::Rect(2.0, 2.0)));
}
