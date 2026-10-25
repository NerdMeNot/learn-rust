struct Point {
    x: i32,
    y: i32,
}

struct Rgb(u8, u8, u8);

enum Shape {
    Circle { center: Point, radius: u32 },
    Rect(Point, Point),
}

fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle {
            center: Point { x: 0, y: 0 },
            radius,
        } => format!("circle at the origin, radius {radius}"),
        Shape::Circle { center, radius } => {
            format!("circle at ({}, {}), radius {radius}", center.x, center.y)
        }
        Shape::Rect(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) => {
            format!("rect {} wide, {} high", x2 - x1, y2 - y1)
        }
    }
}

fn main() {
    // by field name, in any order; a new name after the colon
    let Point { y, x: px } = Point { x: 3, y: -2 };
    println!("px = {px}, y = {y}");

    // `..` skips the fields (or items) you do not name
    let color = Rgb(200, 100, 50);
    let Rgb(red, ..) = color;
    let Rgb(_, green, blue) = color;
    let (first, .., last) = (1, 2, 3, 4);
    println!("red = {red}, green = {green}, blue = {blue}");
    println!("first = {first}, last = {last}");

    // patterns nest as deep as the data does
    let shapes = [
        Shape::Circle {
            center: Point { x: 0, y: 0 },
            radius: 2,
        },
        Shape::Circle {
            center: Point { x: 5, y: 1 },
            radius: 3,
        },
        Shape::Rect(Point { x: 1, y: 1 }, Point { x: 4, y: 3 }),
    ];
    for shape in &shapes {
        println!("{}", describe(shape));
    }

    // an Option of a pair, and a pair of Options
    let found = Some(("ana", 30));
    if let Some((name, age)) = found {
        println!("{name} is {age}");
    }
    match (Some(1), None::<i32>) {
        (Some(a), Some(b)) => println!("both: {a} {b}"),
        (Some(a), None) | (None, Some(a)) => println!("exactly one: {a}"),
        (None, None) => println!("neither"),
    }
}
