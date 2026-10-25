use std::fmt;

#[derive(Debug)]
enum Direction {
    North,
    South,
}

// `impl fmt::Display` is how you say what `{}` prints. Traits come in a later
// lesson: for now, copy the shape.
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Direction::North => "north",
            Direction::South => "south",
        };
        write!(f, "{text}")
    }
}

impl Direction {
    // text in: a name we know, or None
    fn parse(text: &str) -> Option<Direction> {
        match text {
            "north" => Some(Direction::North),
            "south" => Some(Direction::South),
            _ => None,
        }
    }
}

fn main() {
    let d = Direction::North;
    println!("Display: {d}");
    println!("Debug: {d:?}");
    println!("parse(\"south\"): {:?}", Direction::parse("south"));
    println!("parse(\"up\"): {:?}", Direction::parse("up"));
}
