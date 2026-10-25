use std::any::{type_name, type_name_of_val};
use std::mem::size_of_val;

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

fn show_generic<T: Shape>(s: &T) {
    println!("generic: T is {}, area {}", type_name::<T>(), s.area());
}

fn show_dyn(s: &dyn Shape) {
    println!(
        "dyn: type is {}, area {}, real size {}",
        type_name_of_val(s),
        s.area(),
        size_of_val(s)
    );
}

fn main() {
    let sq = Square { side: 3 };
    let rect = Rect { w: 4, h: 6 };

    show_generic(&sq);
    show_generic(&rect);
    show_dyn(&sq);
    show_dyn(&rect);

    let for_square = show_generic::<Square> as *const ();
    let for_rect = show_generic::<Rect> as *const ();
    println!("two copies of show_generic: {}", for_square != for_rect);
}
