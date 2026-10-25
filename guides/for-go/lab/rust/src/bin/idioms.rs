use std::collections::HashMap;
use std::error::Error;

struct Celsius(f64);

struct Counter {
    n: u32,
}

impl Counter {
    fn inc(&mut self) {
        self.n += 1;
    }
}

fn main() {
    let mut x = 5;
    x += 1;
    println!("reassign: {x}");

    let zero = i32::default();
    println!("zero: {zero}");

    let mut sum = 0;
    for i in 0..3 {
        sum += i;
    }
    println!("range: {sum}");

    let words = ["ana", "bo", "cy"];
    let mut total = 0;
    for (i, w) in words.iter().enumerate() {
        total += i * w.len();
    }
    println!("weighted: {total}");

    let day = "sat";
    let kind = match day {
        "sat" | "sun" => "weekend",
        _ => "weekday",
    };
    println!("kind: {kind}");

    let ages = HashMap::from([("ana", 30)]);
    if let Some(v) = ages.get("ana") {
        println!("lookup: found {v}");
    }

    let mut counts: HashMap<&str, u32> = HashMap::new();
    *counts.entry("the").or_insert(0) += 1;
    *counts.entry("the").or_insert(0) += 1;
    println!("count: {}", counts["the"]);

    let mut v = vec![3, 1];
    v.push(2);
    println!("len: {}", v.len());

    let s = "héllo";
    println!("bytes: {}", s.len());
    println!("chars: {}", s.chars().count());
    println!("fields: {}", "a b".split_whitespace().count());

    if let Ok(n) = "42".parse::<i32>() {
        println!("parsed: ok {n}");
    }

    let (a, b) = ("ana", 7);
    let text = format!("{a}-{b}");
    println!("formatted: {text}");

    v.sort();
    println!("smallest: {}", v[0]);

    println!("upper: {}", "ana".to_uppercase());

    let (mut a, mut b) = (1, 2);
    (a, b) = (b, a);
    println!("swapped: {a} {b}");

    let (p, q) = (3, 5);
    println!("min: {}", p.min(q));

    let c = Celsius(21.5);
    println!("newtype: {}", c.0);

    let e: Box<dyn Error> = "bad input".into();
    println!("error: {e}");

    let mut ctr = Counter { n: 0 };
    ctr.inc();
    println!("method: {}", ctr.n);
}
