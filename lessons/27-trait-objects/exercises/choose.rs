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

// every item has the same type, and the caller knows it: a generic (the bound is missing)
fn total_of<T>(items: &[T]) -> u32 {
    todo!("add the bound, then add up the areas")
}

// items of different types, borrowed from the caller's stack, no Box: a trait object
fn total_mixed(items: &[&dyn Shape]) -> u32 {
    todo!("add up the areas")
}

fn main() {
    let squares = [Square { side: 1 }, Square { side: 2 }];
    let rects = vec![Rect { w: 1, h: 2 }, Rect { w: 3, h: 4 }];
    assert_eq!(total_of(&squares), 5);
    assert_eq!(total_of(&rects), 14);
    let nothing: [Square; 0] = [];
    assert_eq!(total_of(&nothing), 0);

    let sq = Square { side: 3 };
    let rect = Rect { w: 4, h: 6 };
    assert_eq!(total_mixed(&[&sq, &rect]), 33);
    assert_eq!(total_mixed(&[&rect]), 24);
    assert_eq!(total_mixed(&[]), 0);

    // nothing was moved: the values are still ours
    assert_eq!(sq.area() + rect.area(), 33);

    println!("all checks passed");
}
