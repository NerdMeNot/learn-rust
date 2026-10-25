#[derive(Debug, Clone, Copy)]
enum Level {
    Low = 1,
    Mid = 5,
    High = 10,
}

#[derive(Debug, Clone, Copy)]
enum Weekday {
    Mon, // 0
    Tue, // 1
    Wed = 10,
    Thu, // 11
}

impl Level {
    // there is no cast from a number to an enum, so write the check yourself
    fn from_number(n: i32) -> Option<Level> {
        match n {
            1 => Some(Level::Low),
            5 => Some(Level::Mid),
            10 => Some(Level::High),
            _ => None,
        }
    }
}

fn main() {
    println!("Mid as i32: {}", Level::Mid as i32);
    println!("High as u8: {}", Level::High as u8);
    println!("Tue as i32: {}", Weekday::Tue as i32);
    println!("Wed as i32: {}", Weekday::Wed as i32);
    println!("Thu as i32: {}", Weekday::Thu as i32);
    println!("from_number(5): {:?}", Level::from_number(5));
    println!("from_number(7): {:?}", Level::from_number(7));
    println!("{:?} {:?}", Level::Low, Weekday::Mon);
}
