trait Shape {
    fn area(&self) -> u32;
    fn grow(&mut self);
    fn clone_box(&self) -> Box<dyn Shape>;
}

#[derive(Clone)]
struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn grow(&mut self) {
        self.side += 1;
    }
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
struct Scene {
    shapes: Vec<Box<dyn Shape>>,
}

fn main() {
    let mut scene = Scene {
        shapes: vec![Box::new(Square { side: 3 })],
    };
    let copy = scene.clone();
    scene.shapes[0].grow();
    println!("original area: {}", scene.shapes[0].area());
    println!("copy area: {}", copy.shapes[0].area());
}
