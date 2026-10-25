// Backs the Rust column of the "you write -> in Rust" tables in lesson.mdx. Never shown.
const X: i32 = 5;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn double_parsed(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = parse(s)?;
    Ok(n * 2)
}

fn main() {
    // a changeable and a fixed variable
    let mut x = 5;
    x = x + 1;
    let y = 5;
    assert_eq!((x, y, X), (6, 5, 5));

    // a function with types
    assert_eq!(add(2, 3), 5);

    // a counted loop: 0..3 gives 0, 1, 2
    let mut seen = Vec::new();
    for i in 0..3 {
        seen.push(i);
    }
    assert_eq!(seen, [0, 1, 2]);

    // a loop over items
    let words = vec!["a", "b"];
    let mut joined = String::new();
    for w in &words {
        joined.push_str(w);
    }
    assert_eq!(joined, "ab");

    // a branch on a value
    let n = 1;
    let s = match n {
        1 => "one",
        _ => "many",
    };
    assert_eq!(s, "one");

    // a lambda
    let inc = |x| x + 1;
    assert_eq!(inc(1), 2);

    // printing with values inside
    let name = "Ana";
    let age = 30;
    assert_eq!(format!("{name} is {age}"), "Ana is 30");

    // passing an error up
    assert_eq!(double_parsed("21"), Ok(42));
    assert!(double_parsed("x").is_err());
    let fn_parse = |s: &str| -> Result<i32, std::num::ParseIntError> {
        let n = s.parse::<i32>()?;
        Ok(n)
    };
    assert_eq!(fn_parse("7"), Ok(7));
    println!("all rows hold");
}
