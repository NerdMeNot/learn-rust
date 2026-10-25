fn main() {
    let day = 6;
    let kind = match day {
        1..=5 => "weekday",
        6 | 7 => "weekend",
        _ => "not a day",
    };
    println!("{kind}");
}
