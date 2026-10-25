use std::error::Error;
use std::io::Write;

fn parse(text: &str) -> Result<i32, Box<dyn Error>> {
    Ok(text.trim().parse::<i32>()?)
}

fn evens_or_all(only_even: bool) -> Box<dyn Iterator<Item = u32>> {
    if only_even {
        Box::new((1..=6).filter(|n| n % 2 == 0))
    } else {
        Box::new(1..=3)
    }
}

fn log_to(out: &mut dyn Write, msg: &str) {
    writeln!(out, "log: {msg}").unwrap();
}

fn main() {
    println!("parse(\" 42 \"): {:?}", parse(" 42 ").ok());
    println!("parse(\"x\"): {}", parse("x").unwrap_err());

    println!("evens: {:?}", evens_or_all(true).collect::<Vec<_>>());
    println!("first three: {:?}", evens_or_all(false).collect::<Vec<_>>());

    let offset = 10;
    let steps: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(move |x| x + offset),
    ];
    println!(
        "closures: {:?}",
        steps.iter().map(|f| f(5)).collect::<Vec<_>>()
    );

    let mut buffer: Vec<u8> = Vec::new();
    log_to(&mut buffer, "into a Vec");
    log_to(&mut std::io::stdout(), "to the terminal");
    println!("buffer holds {} bytes", buffer.len());
}
