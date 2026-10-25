// Backs the "your language -> Rust" rows: every Rust form in those tables, compiled and called.
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::fs::read_to_string as read_all;
use std::io;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut best = &list[0];
    for item in list {
        if item > best {
            best = item;
        }
    }
    best
}

pub fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    if b > a { b } else { a }
}

fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply64(f: impl Fn(i64) -> i64, x: i64) -> i64 {
    f(x)
}

fn apply_f64(f: impl Fn(f64) -> f64, x: f64) -> f64 {
    f(x)
}

fn apply_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn lookup(m: &HashMap<String, i32>, k: &str) -> Option<i32> {
    m.get(k).copied()
}

fn lookup64(m: &HashMap<String, i64>, k: &str) -> Option<i64> {
    m.get(k).copied()
}

fn find(a: &[i32], key: i32) -> Option<&i32> {
    a.iter().find(|n| **n == key)
}

trait Describe {
    fn describe(&self) -> String;
}

struct Square(u32);

impl Describe for Square {
    fn describe(&self) -> String {
        format!("square {}", self.0)
    }
}

fn show(item: &dyn Describe) -> String {
    item.describe()
}

fn show_generic(item: &impl Describe) -> String {
    item.describe()
}

fn run(fail: bool) -> Result<(), Box<dyn Error>> {
    if fail {
        Err("failed")?;
    }
    Ok(())
}

fn lookup_text(m: &HashMap<String, String>, k: &str) -> Option<String> {
    m.get(k).cloned()
}

fn find_copy(a: &[i32], key: i32) -> Option<i32> {
    a.iter().find(|n| **n == key).copied()
}

// two bounds joined with `+`
fn both<T: PartialOrd + Clone>(a: &T, b: &T) -> T {
    if b > a { b.clone() } else { a.clone() }
}

fn read(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn read_short(path: &str) -> io::Result<String> {
    read_all(path)
}

struct Log {
    lines: Vec<String>,
}

impl Log {
    fn add(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }

    fn count(&self) -> usize {
        self.lines.len()
    }
}

fn bump(x: &mut i32) {
    *x += 1;
}

fn sum_all(xs: &[f64]) -> f64 {
    let mut s = 0.0;
    for x in xs {
        s += x;
    }
    s
}

fn push_one(xs: &mut Vec<f64>) {
    xs.push(1.0);
}

mod shapes {
    pub fn area(w: u32, h: u32) -> u32 {
        w * h
    }
}

fn main() {
    assert_eq!(*largest(&[3, 9, 4]), 9);
    assert_eq!(*largest(&["pear", "fig"]), "pear");
    assert_eq!(max_of(1.5, 0.5), 1.5);
    assert_eq!(apply(|x| x + 1, 1), 2);
    assert_eq!(apply64(|x| x * 2, 4), 8);
    assert_eq!(apply_f64(|x| x / 2.0, 3.0), 1.5);
    fn inc(x: i32) -> i32 {
        x + 1
    }
    assert_eq!(apply_ptr(inc, 1), 2);
    // a closure that captures cannot be a plain `fn` pointer, but `impl Fn` accepts it
    let offset = 10;
    assert_eq!(apply(|x| x + offset, 1), 11);

    let mut m = HashMap::new();
    m.insert("a".to_string(), 1);
    assert_eq!((lookup(&m, "a"), lookup(&m, "b")), (Some(1), None));
    assert_eq!((find(&[1, 2], 2), find(&[1, 2], 5)), (Some(&2), None));
    assert_eq!(show(&Square(2)), "square 2");
    assert_eq!(show_generic(&Square(3)), "square 3");
    assert!(run(true).is_err() && run(false).is_ok());
    let mut ms = HashMap::new();
    ms.insert("a".to_string(), "x".to_string());
    assert_eq!(lookup_text(&ms, "a"), Some("x".to_string()));
    assert_eq!(find_copy(&[1, 2], 2), Some(2));
    assert_eq!(both(&1, &2), 2);
    assert!(read("/nope").is_err());
    assert!(read_short("/nope").is_err());

    let mut m64 = HashMap::new();
    m64.insert("a".to_string(), 1i64);
    assert_eq!((lookup64(&m64, "a"), lookup64(&m64, "b")), (Some(1), None));

    let mut log = Log { lines: Vec::new() };
    log.add("hello");
    assert_eq!(log.count(), 1);
    let mut n = 1;
    bump(&mut n);
    assert_eq!(n, 2);
    let mut v = vec![2.0];
    push_one(&mut v);
    assert_eq!(sum_all(&v), 3.0);
    assert_eq!(shapes::area(2, 3), 6);
    println!("rows check out");
}
