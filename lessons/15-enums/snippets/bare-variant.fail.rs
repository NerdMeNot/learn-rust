enum Direction {
    North,
    South,
}

fn is_north(d: Direction) -> bool {
    match d {
        North => true,
        South => false,
    }
}

fn main() {
    println!("{}", is_north(Direction::South));
}
