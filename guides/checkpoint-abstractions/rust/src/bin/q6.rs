trait Shape {
    fn area(&self) -> f64;
    fn scaled(&self, by: f64) -> Self;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn scaled(&self, by: f64) -> Self {
        Square(self.0 * by)
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square(2.0))];
    println!("{}", shapes[0].area());
}
