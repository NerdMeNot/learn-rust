enum Shape {
    Circle { radius: f64 },
    Rect { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 }, // the three side lengths
}

impl Shape {
    /// Circle: 3.14 * r * r. Rect: width * height.
    /// Triangle: Heron's formula, sqrt(s * (s - a) * (s - b) * (s - c)) with
    /// s = (a + b + c) / 2. If the sides cannot make a triangle (one side is
    /// at least as long as the other two together), the area is 0.0.
    fn area(&self) -> f64 {
        todo!("match on self, one arm per variant")
    }
}

fn close(x: f64, y: f64) -> bool {
    (x - y).abs() < 1e-9
}

fn main() {
    assert!(close(Shape::Circle { radius: 1.0 }.area(), 3.14));
    assert!(close(Shape::Circle { radius: 0.0 }.area(), 0.0));
    assert!(close(
        Shape::Rect {
            width: 2.0,
            height: 3.0
        }
        .area(),
        6.0
    ));
    assert!(close(
        Shape::Triangle {
            a: 3.0,
            b: 4.0,
            c: 5.0
        }
        .area(),
        6.0
    ));
    // sides that cannot make a triangle
    assert!(close(
        Shape::Triangle {
            a: 1.0,
            b: 2.0,
            c: 3.0
        }
        .area(),
        0.0
    ));
    assert!(close(
        Shape::Triangle {
            a: 1.0,
            b: 1.0,
            c: 5.0
        }
        .area(),
        0.0
    ));
    println!("all checks passed");
}
