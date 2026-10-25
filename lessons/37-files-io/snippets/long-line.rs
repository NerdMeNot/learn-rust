use std::io::{BufRead, BufReader};

fn main() {
    let mut data = "a".repeat(70_000);
    data.push_str("\nshort\n");

    let lengths: Vec<usize> = BufReader::new(data.as_bytes())
        .lines()
        .map(|l| l.unwrap().len())
        .collect();
    println!("line lengths: {lengths:?}");
}
