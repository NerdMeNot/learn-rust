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

fn show(s: &dyn Shape) {
    println!("area: {}", s.area());
}

fn main() {
    let boxed: Box<dyn Shape> = Box::new(Rect { w: 4, h: 6 });
    show(&boxed);
}
