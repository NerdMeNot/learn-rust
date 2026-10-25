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

struct Noisy(&'static str);

impl Shape for Noisy {
    fn area(&self) -> u32 {
        self.0.len() as u32
    }
}

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("dropping {}", self.0);
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Noisy("first")),
        Box::new(Square { side: 2 }),
        Box::new(Noisy("third")),
        Box::new(Rect { w: 1, h: 1 }),
    ];
    println!(
        "areas: {:?}",
        shapes.iter().map(|s| s.area()).collect::<Vec<_>>()
    );
    drop(shapes);
    println!("done");
}
