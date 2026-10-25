use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
enum ParsePointError {
    Shape,
    Number,
}

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Point, ParsePointError> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError::Shape)?;
        let x = x.trim().parse().map_err(|_| ParsePointError::Number)?;
        let y = y.trim().parse().map_err(|_| ParsePointError::Number)?;
        Ok(Point { x, y })
    }
}

fn main() {
    println!("{:?}", "3, -4".parse::<Point>());
    println!("{:?}", Point::from_str("7,8"));
    println!("{:?}", "7;8".parse::<Point>());
    println!("{:?}", "7,x".parse::<Point>());

    // parse works for any type with FromStr, including the built-in ones
    println!("{:?} {:?}", "42".parse::<u8>(), "-1".parse::<u8>());
    println!("{:?}", "true".parse::<bool>());
}
