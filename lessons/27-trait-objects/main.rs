trait Shape {
    fn name(&self) -> String;
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
    fn name(&self) -> String {
        String::from("square")
    }
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

impl Shape for Rect {
    fn name(&self) -> String {
        String::from("rect")
    }
    fn area(&self) -> u32 {
        self.w * self.h
    }
}

fn main() {
    // a Vec holds one type, so the list holds boxes: owned pointers to heap values
    let shapes: Vec<Box<dyn Shape>> =
        vec![Box::new(Square { side: 3 }), Box::new(Rect { w: 4, h: 6 })];

    let mut total = 0;
    for s in &shapes {
        println!("{}: {}", s.name(), s.area());
        total += s.area();
    }
    println!("total: {total}");
}
