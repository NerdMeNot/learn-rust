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

fn make(kind: &str) -> Option<Box<dyn Shape>> {
    match kind {
        "square" => Some(Box::new(Square { side: 3 })),
        "rect" => Some(Box::new(Rect { w: 4, h: 6 })),
        _ => None,
    }
}

fn main() {
    for kind in ["square", "rect", "circle"] {
        match make(kind) {
            Some(shape) => println!("{kind}: area {}", shape.area()),
            None => println!("{kind}: unknown shape"),
        }
    }
}
