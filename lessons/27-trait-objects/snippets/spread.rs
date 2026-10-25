use std::any::type_name;

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

// generic: the type parameter has to be named by everything that holds a Canvas
struct Canvas<S: Shape> {
    shapes: Vec<S>,
}

struct App<S: Shape> {
    canvas: Canvas<S>,
}

// dyn: the shapes' types are not part of the type
struct DynCanvas {
    shapes: Vec<Box<dyn Shape>>,
}

struct DynApp {
    canvas: DynCanvas,
}

fn main() {
    let a = App {
        canvas: Canvas {
            shapes: vec![Square { side: 3 }],
        },
    };
    let b = App {
        canvas: Canvas {
            shapes: vec![Rect { w: 4, h: 6 }],
        },
    };
    println!("generic a: {}", type_name::<App<Square>>());
    println!("generic b: {}", type_name::<App<Rect>>());
    println!("areas: {} and {}", a.canvas.shapes[0].area(), b.canvas.shapes[0].area());

    let c = DynApp {
        canvas: DynCanvas {
            shapes: vec![Box::new(Square { side: 3 }), Box::new(Rect { w: 4, h: 6 })],
        },
    };
    println!("dyn: {}", type_name::<DynApp>());
    println!("dyn areas: {} and {}", c.canvas.shapes[0].area(), c.canvas.shapes[1].area());
}
