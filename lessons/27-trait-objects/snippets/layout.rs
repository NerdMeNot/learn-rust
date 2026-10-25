use std::mem::{align_of_val, size_of, size_of_val};

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
    println!("size_of Square: {}", size_of::<Square>());
    println!("size_of Rect: {}", size_of::<Rect>());
    println!("size_of &Rect: {}", size_of::<&Rect>());
    println!("size_of &dyn Shape: {}", size_of::<&dyn Shape>());
    println!("size_of Box<Rect>: {}", size_of::<Box<Rect>>());
    println!("size_of Box<dyn Shape>: {}", size_of::<Box<dyn Shape>>());
    println!(
        "size_of Option<Box<dyn Shape>>: {}",
        size_of::<Option<Box<dyn Shape>>>()
    );
    println!("size_of &[u32]: {}", size_of::<&[u32]>());
    println!("size_of &str: {}", size_of::<&str>());

    let shapes: Vec<Box<dyn Shape>> =
        vec![Box::new(Square { side: 3 }), Box::new(Rect { w: 4, h: 6 })];
    for (i, s) in shapes.iter().enumerate() {
        let shape: &dyn Shape = &**s;
        println!(
            "shapes[{i}]: area {}, size_of_val {}, align_of_val {}",
            shape.area(),
            size_of_val(shape),
            align_of_val(shape)
        );
    }

    let sq = Square { side: 3 };
    let plain: &Square = &sq;
    let erased: &dyn Shape = plain;
    let same = std::ptr::addr_eq(plain as *const Square, erased as *const dyn Shape);
    println!("same data address: {same} (area {})", erased.area());
}
