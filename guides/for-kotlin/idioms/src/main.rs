use std::collections::HashMap;
use std::fmt::Display;
use std::num::ParseIntError;
use std::sync::LazyLock;

#[derive(Debug, Clone, Copy)]
struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    fn origin() -> Pt {
        Pt { x: 0, y: 0 }
    }
}

enum Shape {
    Circle(f64),
    Dot,
}

static ANSWER: LazyLock<i32> = LazyLock::new(|| 6 * 7);

fn f(s: &str) -> Result<i32, ParseIntError> {
    s.parse()
}

// Prints a missing value the way Kotlin does, so the two programs print the same lines.
fn show<T: Display>(o: Option<T>) -> String {
    match o {
        Some(v) => v.to_string(),
        None => "null".to_string(),
    }
}

fn main() {
    let xs = [3, 1, 2];
    println!("2 {:?}", xs.iter().map(|x| x * 2).collect::<Vec<_>>());
    println!("3 {}", xs.iter().filter(|&&x| x > 1).sum::<i32>());
    let mut sb = String::new();
    for (i, x) in xs.iter().enumerate() {
        sb.push_str(&format!("{i}:{x} "));
    }
    println!("4 {}", sb.trim());
    let name = "ana";
    let age = 30;
    let s = format!("{name} is {age}");
    println!("5 {s}");
    println!("6 {}", "x".parse::<i32>().unwrap_or(0));
    let mut m: HashMap<&str, Vec<i32>> = HashMap::new();
    m.entry("a").or_default().push(1);
    println!("7 {:?}", m["a"]);
    let pair = (1, "one");
    let (a, b) = pair;
    println!("8 {a} {b}");
    let o = Shape::Circle(2.0);
    let r = if let Shape::Circle(r) = o { r } else { 0.0 };
    println!("9 {r:?}");
    let _dot = Shape::Dot;
    let x: Option<i32> = Some(4);
    println!("10 {}", show(x.map(|v| v + 1)));
    println!("11 {}", show(xs.iter().find(|&&x| x > 1)));
    let mut ys = ["bb", "a", "ccc"];
    ys.sort_by_key(|s| s.len());
    println!("12 {}", ys.join(" "));
    let p = Pt { x: 1, y: 2 };
    let q = Pt { x: 5, ..p };
    println!("13 {},{}", q.x, q.y);
    println!("14 {},{}", Pt::origin().x, Pt::origin().y);
    println!("15 {}", *ANSWER);
    let n = f("x").unwrap_or(0);
    println!("16 {n}");
    let words = ["a", "b", "c"];
    println!("17 {}", words.join(", "));
    let mut r1 = String::new();
    for i in 1..3 {
        r1.push_str(&i.to_string());
    }
    let mut r2 = String::new();
    for i in 1..=3 {
        r2.push_str(&i.to_string());
    }
    println!("18 {r1} {r2}");
    let k = 2;
    let size = match k {
        1 | 2 => "low",
        _ => "high",
    };
    println!("19 {size}");
    println!("22 {}", &"kotlin"[1..3]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer_is_forty_two() {
        assert_eq!(*ANSWER, 42);
    }

    #[test]
    fn missing_prints_null() {
        assert_eq!(show::<i32>(None), "null");
    }
}
