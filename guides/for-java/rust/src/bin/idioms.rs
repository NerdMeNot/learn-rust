use std::collections::HashMap;
use std::sync::Mutex;
use std::thread;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Circle(f64),
    Square,
}

trait Greeter {
    fn hi(&self) -> String {
        String::from("hi")
    }
}

struct English;

impl Greeter for English {}

fn main() {
    let x = 1;
    println!("1: {x}");

    let mut count = 0;
    count += 1;
    println!("2: {count}");

    let n = 7;
    let size = if n > 5 { "big" } else { "small" };
    println!("3: {size}");

    let level = match n {
        1 | 2 => "low",
        _ => "high",
    };
    println!("4: {level}");

    let len = Some("hi").map(|s| s.len()).unwrap_or(0);
    println!("5: {len}");

    let mut ages = HashMap::new();
    ages.insert("ana", 31);
    let bo = ages.get("bo").copied().unwrap_or(0);
    println!("6: {bo}");

    let mut counts = HashMap::new();
    *counts.entry("a").or_insert(0) += 1;
    println!("7: {}", counts["a"]);

    let nums = vec![1, 2, 3, 4];
    let squares = nums
        .iter()
        .filter(|n| **n % 2 == 0)
        .map(|n| n * n)
        .collect::<Vec<_>>();
    println!("8: {squares:?}");

    let names = vec!["ana", "bo"];
    let mut seen = String::new();
    for s in &names {
        seen.push_str(s);
    }
    println!("9: {seen}");

    println!("10: {}", names.join(", "));

    let word = "héllo";
    println!(
        "11: {} chars, len() is {}",
        word.chars().count(),
        word.len()
    );

    let s = "eighty";
    let parsed = s.parse::<i32>().unwrap_or(-1);
    println!("12: {parsed}");

    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };
    println!("13: {}", a == b);

    for shape in [Shape::Circle(2.5), Shape::Square] {
        if let Shape::Circle(r) = shape {
            println!("14: circle, radius {r}");
        } else {
            println!("14: not a circle");
        }
    }

    println!("15: {}", English.hi());

    let original = vec!["a", "b"];
    let mut copy = original.clone();
    copy.push("c");
    println!("16: {} {}", original.len(), copy.len());

    println!("17: {}", thread::spawn(|| 5).join().unwrap());

    let count = Mutex::new(0);
    *count.lock().unwrap() += 1;
    println!("18: {}", count.lock().unwrap());

    println!("19: {}", 3.9_f64 as i32);

    println!("20: {}", format!("x={n}"));
}
