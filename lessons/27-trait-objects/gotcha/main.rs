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
    let shapes: Vec<Shape> = vec![Square { side: 3 }, Rect { w: 4, h: 6 }];

    let total: u32 = shapes.iter().map(|s| s.area()).sum();
    println!("total: {total}");
}
