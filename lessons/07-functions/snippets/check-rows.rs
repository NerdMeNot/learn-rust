// Backs the "in Rust" cells of the tables in lesson.mdx. Never shown.
fn greet(_name: &str) {}

fn greet_explicit(_name: &str) -> () {}

fn divmod(a: i32, b: i32) -> (i32, i32) {
    (a / b, a % b)
}

fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn area(w: u32, h: Option<u32>) -> u32 {
    w * h.unwrap_or(1)
}

fn apply_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply_impl(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn bump(x: &mut i32) {
    *x += 1;
}

const fn square(n: usize) -> usize {
    n * n
}

fn double_text(s: &str) -> Result<i32, String> {
    let n: i32 = s.parse().map_err(|_| format!("not a number: {s}"))?;
    Ok(n * 2)
}

fn ignore(_x: i32) {}

struct Size {
    w: u32,
    h: u32,
}

fn area_of(s: Size) -> u32 {
    s.w * s.h
}

struct Counter {
    n: i32,
}

impl Counter {
    fn inc(&mut self) {
        self.n += 1;
    }
}

fn add_item(xs: &mut Vec<i32>) {
    xs.push(1);
}

fn text_len(s: &str) -> usize {
    s.len()
}

fn show(x: impl std::fmt::Display) -> String {
    format!("{x}")
}

fn total(opts: &std::collections::HashMap<String, i32>) -> i32 {
    opts.values().sum()
}

fn never() -> ! {
    panic!("stop")
}

fn later() -> i32 {
    todo!("not yet")
}

fn main() {
    // `fn f()` and `fn f() -> ()` have the same type.
    let a: fn(&str) = greet;
    let b: fn(&str) -> () = greet;
    let c: fn(&str) = greet_explicit;
    a("x");
    b("x");
    c("x");

    assert_eq!(divmod(7, 2), (3, 1));
    let (q, r) = divmod(7, 2);
    assert_eq!((q, r), (3, 1));

    assert_eq!(sum(&[1, 2, 3]), 6);
    assert_eq!(sum(&[]), 0);
    let v = vec![4, 5];
    assert_eq!(sum(&v), 9);

    assert_eq!(area(3, None), 3);
    assert_eq!(area(3, Some(4)), 12);

    let offset = 5;
    assert_eq!(apply_ptr(|n| n + 1, 1), 2);
    assert_eq!(apply_impl(|n| n + offset, 1), 6);
    let f: fn(i32) -> i32 = |x| x + 1;
    assert_eq!(f(1), 2);

    let mut n = 1;
    bump(&mut n);
    assert_eq!(n, 2);

    assert_eq!(double_text("21"), Ok(42));
    assert_eq!(double_text("x"), Err("not a number: x".to_string()));

    ignore(1);
    let (_, r) = divmod(7, 2);
    assert_eq!(r, 1);

    assert_eq!(area_of(Size { w: 3, h: 4 }), 12);

    let mut c = Counter { n: 0 };
    c.inc();
    assert_eq!(c.n, 1);

    let mut xs = vec![0];
    add_item(&mut xs);
    assert_eq!(xs, [0, 1]);
    assert_eq!(text_len(&String::from("abc")), 3);
    assert_eq!(show(5), "5");
    assert_eq!(show("x"), "x");

    let mut opts = std::collections::HashMap::new();
    opts.insert("retries".to_string(), 3);
    opts.insert("delay".to_string(), 4);
    assert_eq!(total(&opts), 7);

    // `!` and `todo!()` type-check where a function is expected.
    let _f: fn() -> ! = never;
    let _g: fn() -> i32 = later;

    let grid = [0u8; square(3)];
    assert_eq!(grid.len(), 9);
    println!("all checks passed");
}
