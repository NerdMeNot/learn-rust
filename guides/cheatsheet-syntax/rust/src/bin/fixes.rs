use std::num::ParseIntError;

struct Rect {
    width: u32,
    height: u32,
}

fn area(r: &Rect) -> u32 {
    r.width * r.height
}

trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("int {self}")
    }
}

impl Show for &str {
    fn show(&self) -> String {
        format!("text {self}")
    }
}

fn parse(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.parse()?;
    Ok(n)
}

fn main() {
    // 1  x++
    let mut x = 5;
    x += 1;
    println!("1  x += 1            -> {x}");

    // 2  c ? a : b
    let big = true;
    let n = if big { 10 } else { 1 };
    println!("2  if big {{ 10 }} else {{ 1 }} -> {n}");

    // 3  for (i = 0; i < 3; i++)
    let mut seen = Vec::new();
    for i in 0..3 {
        seen.push(i);
    }
    let stepped: Vec<i32> = (0..9).step_by(3).collect();
    println!("3  0..3 -> {seen:?}, (0..9).step_by(3) -> {stepped:?}");

    // 4  null
    let name: Option<String> = None;
    let shown = match &name {
        Some(s) => s.as_str(),
        None => "(none)",
    };
    println!("4  Option<String> None -> {shown}");

    // 5  implicit numeric conversion
    let small: i32 = 7;
    let wide: i64 = i64::from(small);
    let scaled = f64::from(small) * 1.5;
    println!("5  i64::from(7) -> {wide}, f64::from(7) * 1.5 -> {scaled}");

    // 6  named arguments
    let r = Rect { width: 3, height: 4 };
    println!("6  area(&Rect {{ width: 3, height: 4 }}) -> {}", area(&r));

    // 7  overloading
    println!("7  1.show() -> {}, \"a\".show() -> {}", 1.show(), "a".show());

    // 8  switch with fallthrough
    for n in [1, 2, 3, 9] {
        let word = match n {
            1 => "one",
            2 | 3 => "two or three",
            _ => "many",
        };
        println!("8  match {n} -> {word}");
    }

    // 9  try / catch
    println!("9  parse(\"42\") -> {:?}", parse("42"));
    println!("9  parse(\"x\")  -> {:?}", parse("x"));

    // 10  truthiness
    let count = 3;
    let items: Vec<i32> = Vec::new();
    println!("10 count != 0 -> {}, !items.is_empty() -> {}", count != 0, !items.is_empty());

    // 11  string indexing
    let s = String::from("héllo");
    println!(
        "11 chars().next() -> {:?}, as_bytes()[0] -> {}, &s[0..1] -> {:?}",
        s.chars().next(),
        s.as_bytes()[0],
        &s[0..1]
    );

    // 12  == across types
    let k = 1;
    println!("12 k == 1 -> {}, f64::from(k) == 1.0 -> {}", k == 1, f64::from(k) == 1.0);
    println!("12 String == &str -> {}", String::from("a") == "a");
}
