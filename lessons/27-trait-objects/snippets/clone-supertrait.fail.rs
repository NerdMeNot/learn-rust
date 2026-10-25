trait Shape: Clone {
    fn area(&self) -> u32;
}

#[derive(Clone)]
struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square { side: 3 })];
    println!("count: {}", shapes.len());
}
