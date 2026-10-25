#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    Low,
    Mid(u8),
    High,
}

fn main() {
    let mut levels = vec![Level::High, Level::Mid(5), Level::Low, Level::Mid(2)];
    levels.sort();
    println!("{levels:?}");
    println!("{}", Level::Mid(200) < Level::High);
}
