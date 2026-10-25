trait Shape {
    fn area(&self) -> u32;
}

struct Square;

impl Shape for Square {
    fn area(&self) -> u32 {
        4
    }
}

struct Rock;

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Square), Box::new(Rock)];
    for shape in &shapes {
        println!("{}", shape.area());
    }
}
