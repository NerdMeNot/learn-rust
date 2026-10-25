use std::collections::BTreeMap;
use std::sync::Mutex;

enum Shape {
    Circle(f64),
    Square(f64),
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Person {
    pub age: u32,
}

fn main() {
    // 1  a local that changes
    let mut x = 5;
    x += 1;
    println!("1  {x}");

    // 2  null with a default
    let name = std::env::args().nth(1);
    println!("2  {}", name.as_deref().unwrap_or("anon"));

    // 3  call through a maybe-missing value
    let word: Option<&str> = Some("ana");
    println!("3  {:?}", word.map(str::len));

    // 4  parse without an exception
    let text = "42";
    if let Ok(n) = text.parse::<i32>() {
        println!("4  {n}");
    }

    // 5  string interpolation with width and zero padding
    let seven = 7;
    println!("5  [{seven:5}] [{seven:03}]");

    // 6  an iterator pipeline
    let xs = vec![3, 1, 2];
    let doubled = xs
        .iter()
        .filter(|&&v| v > 1)
        .map(|&v| v * 2)
        .collect::<Vec<_>>();
    println!("6  {doubled:?}");

    // 7  sum and max
    println!("7  {} {:?}", xs.iter().sum::<i32>(), xs.iter().max());

    // 8  walk a sorted map
    let ages = BTreeMap::from([("bo", 31), ("ana", 30)]);
    for (k, v) in &ages {
        println!("8  {k}={v}");
    }

    // 9  look a key up
    if let Some(bo) = ages.get("bo") {
        println!("9  {bo}");
    }

    // 10  count words
    let mut counts = BTreeMap::new();
    for w in "a b a".split(' ') {
        *counts.entry(w).or_insert(0) += 1;
    }
    println!("10 a={} b={}", counts["a"], counts["b"]);

    // 11  a match expression
    let size = match x {
        1 => "one",
        _ => "many",
    };
    println!("11 {size}");

    // 12  test the kind of a value and take it apart
    let sh = if x > 5 {
        Shape::Circle(2.0)
    } else {
        Shape::Square(1.0)
    };
    if let Shape::Circle(r) = sh {
        println!("12 radius {r}");
    } else if let Shape::Square(s) = sh {
        println!("12 side {s}");
    }

    // 13  derive equality, and struct update
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, ..p1 };
    println!("13 {} {:?}", p1 == Point { x: 1, y: 2 }, p2);

    // 14  length in characters (and in bytes)
    let accent = "héllo";
    let emoji = "a😀";
    println!(
        "14 {} {} {}",
        accent.chars().count(),
        accent.len(),
        emoji.chars().count()
    );

    // 15  a lock around shared state
    let hits = Mutex::new(0);
    *hits.lock().unwrap() += 1;
    println!("15 {}", hits.lock().unwrap());

    // 16  a field
    let mut person = Person { age: 30 };
    person.age += 1;
    println!("16 {}", person.age);
}
