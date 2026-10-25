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

// "square 3" and "rect 4 6" become shapes; anything else is None
fn parse_shape(spec: &str) -> Option<Box<dyn Shape>> {
    let parts: Vec<&str> = spec.split_whitespace().collect();
    match parts.as_slice() {
        ["square", side] => Some(Box::new(Square {
            side: side.parse().ok()?,
        })),
        ["rect", w, h] => Some(Box::new(Rect {
            w: w.parse().ok()?,
            h: h.parse().ok()?,
        })),
        _ => None,
    }
}

// the shape with the largest area; on a tie, the first one
fn largest(shapes: &[Box<dyn Shape>]) -> Option<&dyn Shape> {
    let mut best: Option<&dyn Shape> = None;
    for shape in shapes {
        match best {
            Some(b) if b.area() >= shape.area() => {}
            _ => best = Some(shape.as_ref()),
        }
    }
    best
}

fn main() {
    assert_eq!(parse_shape("square 3").map(|s| s.area()), Some(9));
    assert_eq!(parse_shape("rect 4 6").map(|s| s.area()), Some(24));
    assert_eq!(
        parse_shape("  rect   4 6 ").map(|s| s.name()),
        Some("rect".to_string())
    );

    // anything that is not a whole shape is None
    assert!(parse_shape("rect 4").is_none());
    assert!(parse_shape("rect 4 6 7").is_none());
    assert!(parse_shape("square 3 4").is_none());
    assert!(parse_shape("square x").is_none());
    assert!(parse_shape("square -1").is_none());
    assert!(parse_shape("hexagon 2").is_none());
    assert!(parse_shape("").is_none());

    let none: Vec<Box<dyn Shape>> = Vec::new();
    assert!(largest(&none).is_none());

    let mixed: Vec<Box<dyn Shape>> = ["square 1", "rect 2 5", "square 3"]
        .iter()
        .filter_map(|s| parse_shape(s))
        .collect();
    assert_eq!(largest(&mixed).map(|s| s.name()), Some("rect".to_string()));

    // a tie goes to the first
    let tie: Vec<Box<dyn Shape>> = ["square 2", "rect 1 4"]
        .iter()
        .filter_map(|s| parse_shape(s))
        .collect();
    assert_eq!(largest(&tie).map(|s| s.name()), Some("square".to_string()));

    println!("all checks passed");
}
