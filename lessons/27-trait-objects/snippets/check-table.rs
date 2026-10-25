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

// a third type, defined later and somewhere else, joins the same list
mod elsewhere {
    pub struct Triangle {
        pub base: u32,
        pub height: u32,
    }

    impl super::Shape for Triangle {
        fn area(&self) -> u32 {
            self.base * self.height / 2
        }
    }
}

// a built-in type can join the list too: the trait is ours, so the impl is allowed
impl Shape for i32 {
    fn area(&self) -> u32 {
        self.unsigned_abs()
    }
}

// a supertrait gives a dyn its toString: Display works through Box<dyn Labelled>
trait Labelled: std::fmt::Display {
    fn id(&self) -> u32;
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "square {}", self.side)
    }
}

impl Labelled for Square {
    fn id(&self) -> u32 {
        self.side
    }
}

fn total_generic<T: Shape>(shapes: &[T]) -> u32 {
    shapes.iter().map(|s| s.area()).sum()
}

fn total_dyn(shapes: &[Box<dyn Shape>]) -> u32 {
    shapes.iter().map(|s| s.area()).sum()
}

enum Closed {
    Square { side: u32 },
    Rect { w: u32, h: u32 },
}

impl Closed {
    fn area(&self) -> u32 {
        match self {
            Closed::Square { side } => side * side,
            Closed::Rect { w, h } => w * h,
        }
    }
}

fn main() {
    // generic: a slice of one type works
    assert_eq!(total_generic(&[Square { side: 1 }, Square { side: 2 }]), 5);

    // dyn: a mix works, and so does a type added from another module
    let mixed: Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 3 }),
        Box::new(Rect { w: 4, h: 6 }),
        Box::new(elsewhere::Triangle { base: 4, height: 5 }),
    ];
    assert_eq!(total_dyn(&mixed), 9 + 24 + 10);

    // a built-in type in the same list
    let with_int: Vec<Box<dyn Shape>> = vec![Box::new(7), Box::new(Square { side: 2 })];
    assert_eq!(total_dyn(&with_int), 7 + 4);

    // Display through a dyn, because Labelled: Display
    let labelled: Box<dyn Labelled> = Box::new(Square { side: 5 });
    assert_eq!(labelled.to_string(), "square 5");
    assert_eq!(labelled.id(), 5);

    // enum: values sit inline, no pointer, the size of the largest variant plus a tag
    let inline = [Closed::Square { side: 3 }, Closed::Rect { w: 4, h: 6 }];
    assert_eq!(
        std::mem::size_of_val(&inline),
        2 * std::mem::size_of::<Closed>()
    );
    assert_eq!(std::mem::size_of::<Closed>(), 12);
    assert_eq!(inline.iter().map(|s| s.area()).sum::<u32>(), 33);

    // shared ownership works with dyn too
    let shared: std::rc::Rc<dyn Shape> = std::rc::Rc::new(Square { side: 2 });
    assert_eq!(shared.area(), 4);
    let atomic: std::sync::Arc<dyn Shape> = std::sync::Arc::new(Rect { w: 2, h: 3 });
    assert_eq!(atomic.area(), 6);

    // a dyn element is the 16-byte pair, whatever it points at
    assert_eq!(std::mem::size_of::<Box<dyn Shape>>(), 16);
    assert_eq!(std::mem::size_of_val(&*mixed), 3 * 16);
}
