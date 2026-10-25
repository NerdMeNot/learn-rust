struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Circle { center: Point, radius: u32 },
    Rect(Point, Point),
    Tag { name: String, at: Point },
}

/// Circle: "circle r=2 at (0, 0)". Rect (the corners can come in either order): "rect 3x2",
/// width then height. Tag: "tag ana at (5, -1)". A circle with radius 0, or a rect with
/// no width or no height, is "empty".
fn describe(shape: &Shape) -> String {
    todo!("match on the shape; each arm names the fields it needs")
}

/// Make a shape bigger in place: a circle's radius goes up by 1, and a rect's second corner
/// moves 1 right and 1 down (x + 1, y + 1). A tag does not change.
fn grow(shape: &mut Shape) {
    todo!("names bound through &mut are &mut references: write *radius += 1")
}

fn main() {
    let at = |x, y| Point { x, y };
    let mut shapes = vec![
        Shape::Circle {
            center: at(0, 0),
            radius: 2,
        },
        Shape::Rect(at(1, 1), at(4, 3)),
        Shape::Rect(at(4, 3), at(1, 1)),
        Shape::Tag {
            name: String::from("ana"),
            at: at(5, -1),
        },
        Shape::Circle {
            center: at(0, 0),
            radius: 0,
        },
        Shape::Rect(at(2, 2), at(2, 9)),
    ];
    let before: Vec<String> = shapes.iter().map(describe).collect();
    assert_eq!(
        before,
        [
            "circle r=2 at (0, 0)",
            "rect 3x2",
            "rect 3x2",
            "tag ana at (5, -1)",
            "empty",
            "empty",
        ]
    );
    // describing only borrows: the tag still owns its name afterwards
    assert_eq!(describe(&shapes[3]), "tag ana at (5, -1)");

    for shape in &mut shapes {
        grow(shape);
    }
    let after: Vec<String> = shapes.iter().map(describe).collect();
    assert_eq!(
        after,
        [
            "circle r=3 at (0, 0)",
            "rect 4x3",
            "rect 2x1",
            "tag ana at (5, -1)",
            "circle r=1 at (0, 0)",
            "rect 1x8",
        ]
    );
    println!("all checks passed");
}
