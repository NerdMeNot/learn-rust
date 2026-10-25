trait Shape {
    fn area(&self) -> u32;
    fn doubled(&self) -> Self
    where
        Self: Sized;
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn doubled(&self) -> Self {
        Square {
            side: self.side * 2,
        }
    }
}

fn main() {
    let shape: Box<dyn Shape> = Box::new(Square { side: 3 });
    let bigger = shape.doubled();
    println!("area: {}", bigger.area());
}
