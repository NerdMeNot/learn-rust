use std::fmt::Debug;

trait Shape {
    fn area(&self) -> u32;
    fn doubled(&self) -> Self;
    fn describe<T: Debug>(&self, extra: T) -> String;
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
    fn describe<T: Debug>(&self, extra: T) -> String {
        format!("square {} ({extra:?})", self.side)
    }
}

fn main() {
    let shape: Box<dyn Shape> = Box::new(Square { side: 3 });
    println!("area: {}", shape.area());
}
