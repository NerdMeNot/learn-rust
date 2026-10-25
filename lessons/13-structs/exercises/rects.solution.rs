#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq)]
struct Rect {
    top_left: Point,
    width: u32,
    height: u32,
}

/// Width times height, for any rectangle, including the huge one at the end of `main`.
fn area(r: &Rect) -> u64 {
    u64::from(r.width) * u64::from(r.height)
}

/// True when `p` is inside `r`. The left and top edges count as inside; the right and
/// bottom edges do not. A rectangle with no width or no height contains nothing.
fn contains(r: &Rect, p: Point) -> bool {
    let left = i64::from(r.top_left.x);
    let top = i64::from(r.top_left.y);
    let right = left + i64::from(r.width);
    let bottom = top + i64::from(r.height);
    let (x, y) = (i64::from(p.x), i64::from(p.y));
    left <= x && x < right && top <= y && y < bottom
}

/// Move the rectangle by `dx` and `dy`, which may be negative.
fn shift(r: &mut Rect, dx: i32, dy: i32) {
    r.top_left.x += dx;
    r.top_left.y += dy;
}

fn main() {
    let mut r = Rect {
        top_left: Point { x: 2, y: 3 },
        width: 4,
        height: 2,
    };
    assert_eq!(area(&r), 8);

    assert!(contains(&r, Point { x: 2, y: 3 })); // top-left corner
    assert!(contains(&r, Point { x: 5, y: 4 })); // last cell inside
    assert!(!contains(&r, Point { x: 6, y: 4 })); // right edge is outside
    assert!(!contains(&r, Point { x: 5, y: 5 })); // bottom edge is outside
    assert!(!contains(&r, Point { x: 1, y: 3 }));

    shift(&mut r, -5, 1);
    assert_eq!(r.top_left, Point { x: -3, y: 4 });
    assert!(contains(&r, Point { x: -3, y: 4 }));
    assert!(contains(&r, Point { x: 0, y: 5 }));
    assert!(!contains(&r, Point { x: 1, y: 5 }));

    let flat = Rect {
        top_left: Point { x: 0, y: 0 },
        width: 5,
        height: 0,
    };
    assert_eq!(area(&flat), 0);
    assert!(!contains(&flat, Point { x: 0, y: 0 }));

    let huge = Rect {
        top_left: Point {
            x: i32::MAX,
            y: i32::MIN,
        },
        width: u32::MAX,
        height: u32::MAX,
    };
    assert_eq!(area(&huge), 18_446_744_065_119_617_025);
    assert!(contains(
        &huge,
        Point {
            x: i32::MAX,
            y: i32::MIN
        }
    ));

    println!("all checks passed");
}
