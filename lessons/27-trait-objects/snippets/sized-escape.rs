use std::fmt::Debug;

trait Shape {
    fn area(&self) -> u32;
    fn doubled(&self) -> Self
    where
        Self: Sized;
    fn describe<T: Debug>(&self, extra: T) -> String
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
    fn describe<T: Debug>(&self, extra: T) -> String {
        format!("square {} ({extra:?})", self.side)
    }
}

fn main() {
    let sq = Square { side: 3 };
    println!("on the concrete type: {}", sq.describe("hi"));
    println!("doubled area: {}", sq.doubled().area());

    let shape: Box<dyn Shape> = Box::new(sq);
    println!("through dyn: area {}", shape.area());
}
