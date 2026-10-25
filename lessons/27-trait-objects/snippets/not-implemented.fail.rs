trait Shape {
    fn area(&self) -> u32;
}

struct Square {
    side: u32,
}

struct Rect {
    w: u32,
    h: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

impl Shape for Rect {
    fn area(&self) -> u32 {
        self.w * self.h
    }
}

struct Rock;

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square { side: 3 }), Box::new(Rock)];
    println!("count: {}", shapes.len());
}
