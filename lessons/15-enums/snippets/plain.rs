#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn turn_right(d: Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
    }
}

fn main() {
    let heading = Direction::North;
    println!("start: {:?}", heading);
    let heading = turn_right(heading);
    println!("after one turn: {:?}", heading);
}
