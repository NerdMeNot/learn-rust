#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn opposite(d: Direction) -> Direction {
    // `use` inside the function brings the variant names into scope here only
    use Direction::*;
    match d {
        North => South,
        South => North,
        East => West,
        West => East,
    }
}

fn main() {
    println!("opposite of East: {:?}", opposite(Direction::East));
}
