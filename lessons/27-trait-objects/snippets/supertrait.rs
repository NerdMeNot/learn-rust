use std::fmt::Debug;

trait Shape: Debug {
    fn area(&self) -> u32;
}

#[derive(Debug)]
struct Square {
    side: u32,
}

#[derive(Debug)]
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
    let shapes: Vec<Box<dyn Shape>> =
        vec![Box::new(Square { side: 3 }), Box::new(Rect { w: 4, h: 6 })];
    println!("{shapes:?}");
    for s in &shapes {
        println!("{s:?} has area {}", s.area());
    }
}
