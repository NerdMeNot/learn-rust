use std::any::Any;

trait Shape: Any {
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
    let shapes: Vec<Box<dyn Shape>> =
        vec![Box::new(Square { side: 3 }), Box::new(Rect { w: 4, h: 6 })];
    for s in &shapes {
        // a Shape is also an Any, because Shape: Any
        let any: &dyn Any = s.as_ref();
        match any.downcast_ref::<Rect>() {
            Some(r) => println!("a Rect, {} wide and {} high", r.w, r.h),
            None => println!("not a Rect (area {})", s.area()),
        }
    }

    // the trap: this asks about the Box itself, not about what it holds
    let boxed_itself: &dyn Any = &shapes[1];
    println!("the Box is a Rect: {}", boxed_itself.is::<Rect>());
    println!(
        "the Box is a Box<dyn Shape>: {}",
        boxed_itself.is::<Box<dyn Shape>>()
    );
}
