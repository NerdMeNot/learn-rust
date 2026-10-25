use std::f64::consts::PI;

trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
    fn describe(&self) -> String {
        format!("{}: {:.2}", self.name(), self.area())
    }
}

struct Circle {
    r: f64,
}

struct Square {
    side: f64,
}

impl Shape for Circle {
    fn name(&self) -> String {
        "circle".to_string()
    }
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
}

impl Shape for Square {
    fn name(&self) -> String {
        "square".to_string()
    }
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> =
        vec![Box::new(Circle { r: 1.0 }), Box::new(Square { side: 2.0 })];
    for s in &shapes {
        println!("{}", s.describe());
    }
}
