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

fn main() {
    let shapes: Vec<dyn Shape> = vec![Square { side: 3 }, Rect { w: 4, h: 6 }];
    println!("count: {}", shapes.len());
}
