trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> u32;
    fn scale(&mut self, by: u32);

    // a default method works through a dyn too
    fn describe(&self) -> String {
        format!("{} with area {}", self.name(), self.area())
    }
}

struct Square {
    side: u32,
}

struct Rect {
    w: u32,
    h: u32,
}

impl Shape for Square {
    fn name(&self) -> String {
        String::from("square")
    }
    fn area(&self) -> u32 {
        self.side * self.side
    }
    fn scale(&mut self, by: u32) {
        self.side *= by;
    }
}

impl Shape for Rect {
    fn name(&self) -> String {
        String::from("rect")
    }
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn scale(&mut self, by: u32) {
        self.w *= by;
        self.h *= by;
    }
}

// borrowed, read-only: any Shape, behind a shared reference
fn show(s: &dyn Shape) {
    println!("show: {}", s.describe());
}

// borrowed, changeable: the same, behind a &mut
fn double(s: &mut dyn Shape) {
    s.scale(2);
}

// owned: a Box owns a heap value; the list owns its boxes
fn total(shapes: &[Box<dyn Shape>]) -> u32 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let mut sq = Square { side: 3 };
    show(&sq);
    double(&mut sq);
    show(&sq);

    // &dyn needs no heap: two references to values in main's own frame
    let rect = Rect { w: 4, h: 6 };
    let on_stack: [&dyn Shape; 2] = [&sq, &rect];
    println!("on the stack: {} shapes", on_stack.len());

    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(sq), Box::new(rect)];
    for s in &shapes {
        println!("list: {}", s.describe());
    }
    println!("total area: {}", total(&shapes));

    // two ways to borrow the shape inside a box as a plain &dyn Shape
    show(shapes[1].as_ref());
    show(&*shapes[0]);
}
