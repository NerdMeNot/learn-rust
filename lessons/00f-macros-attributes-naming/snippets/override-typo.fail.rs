trait Shape {
    fn area(&self) -> f64;
}

struct Square(f64);

impl Shape for Square {
    fn area(&self) -> f64 {
        self.0 * self.0
    }
    fn perimiter(&self) -> f64 {
        4.0 * self.0
    }
}

fn main() {
    println!("area: {}", Square(2.0).area());
}
