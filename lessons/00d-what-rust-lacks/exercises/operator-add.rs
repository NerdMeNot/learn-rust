use std::ops::Add;

// C++: Point operator+(Point a, Point b). In Rust the operator is a trait:
// `impl Add for Point` defines what `a + b` means.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        todo!("add the two x values and the two y values")
    }
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 3, y: 4 };
    assert_eq!(a + b, Point { x: 4, y: 6 });
    // a and b are Copy, so they can still be used after the sum.
    assert_eq!(a + a, Point { x: 2, y: 4 });
    assert_eq!(b + Point { x: -3, y: -4 }, Point { x: 0, y: 0 });
    println!("all checks passed");
}
