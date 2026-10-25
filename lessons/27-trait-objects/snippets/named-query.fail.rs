trait Shape {
    fn area(&self) -> u32;
}

trait Named {
    fn name(&self) -> String;
}

struct Square {
    side: u32,
}

impl Shape for Square {
    fn area(&self) -> u32 {
        self.side * self.side
    }
}

impl Named for Square {
    fn name(&self) -> String {
        String::from("square")
    }
}

fn label(s: &dyn Shape) -> String {
    // the Go habit: ask the shape whether it also has another interface's methods
    (s as &dyn Named).name()
}

fn main() {
    println!("{}", label(&Square { side: 3 }));
}
