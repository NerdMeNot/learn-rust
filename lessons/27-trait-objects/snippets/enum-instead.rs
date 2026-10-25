use std::mem::size_of;

enum Shape {
    Square { side: u32 },
    Rect { w: u32, h: u32 },
}

impl Shape {
    fn area(&self) -> u32 {
        match self {
            Shape::Square { side } => side * side,
            Shape::Rect { w, h } => w * h,
        }
    }
}

fn main() {
    let shapes = vec![Shape::Square { side: 3 }, Shape::Rect { w: 4, h: 6 }];
    for s in &shapes {
        match s {
            Shape::Rect { w, h } => println!("a Rect, {w} wide and {h} high"),
            Shape::Square { .. } => println!("not a Rect (area {})", s.area()),
        }
    }
    println!("size_of Shape (the enum): {}", size_of::<Shape>());
}
