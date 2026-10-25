trait Shape {
    fn area(&self) -> u32;
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

#[derive(Clone)]
struct Scene {
    shapes: Vec<Box<dyn Shape>>,
}

fn main() {
    let scene = Scene {
        shapes: vec![Box::new(Square { side: 3 })],
    };
    let copy = scene.clone();
    println!("copied {} shapes", copy.shapes.len());
}
