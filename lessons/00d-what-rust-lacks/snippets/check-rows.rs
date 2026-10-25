// Backs the "In Rust" cells of the tables in lesson.mdx. Never shown.
use std::collections::HashMap;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
}

trait Speak {
    fn name(&self) -> String;
    fn hello(&self) -> String {
        format!("hi {}", self.name())
    }
}

struct Dog;

impl Speak for Dog {
    fn name(&self) -> String {
        "Rex".to_string()
    }
}

fn greet(name: &str, greeting: Option<&str>) -> String {
    format!("{}, {name}!", greeting.unwrap_or("Hello"))
}

fn parse(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n = s.parse::<i32>()?;
    Ok(n + 1)
}

fn main() {
    // x++ and x--
    let mut x = 5;
    x += 1;
    x -= 1;
    assert_eq!(x, 5);

    // the ternary operator
    let big = if x > 3 { "big" } else { "small" };
    assert_eq!(big, "big");

    // counted loops
    let mut sum = 0;
    for i in 0..4 {
        sum += i;
    }
    assert_eq!(sum, 6);

    // do { } while: the body runs once even when the test is false at the start
    let mut runs = 0;
    loop {
        runs += 1;
        if runs >= 1 {
            break;
        }
    }
    assert_eq!(runs, 1);

    // Option instead of null
    let missing: Option<&str> = None;
    assert!(missing.is_none());
    let present = Some(3);
    if let Some(n) = present {
        assert_eq!(n, 3);
    }

    // numbers: explicit conversions
    let small: i32 = 5;
    let other: i64 = 7;
    assert_eq!(i64::from(small) + other, 12);
    assert_eq!(small as i64 + other, 12);
    assert_eq!(u64::try_from(small).unwrap(), 5);
    assert!(u64::try_from(-1i32).is_err());
    assert_eq!(f64::from(small) * 1.5, 7.5);
    assert_eq!(small as f64 * 1.5, 7.5);
    assert!(i64::from(small) != other);
    assert!(1 as f64 == 1.0);
    // a String and a &str can be compared; two number types cannot
    assert!(String::from("a") == "a");

    // truthiness: the explicit tests
    let name = String::from("Ada");
    let items: Vec<i32> = Vec::new();
    assert!(!name.is_empty());
    assert!(items.is_empty());
    assert!(4 != 0);
    assert!(Some(3).is_some());

    // ! is logical not on bool and bitwise not on integers
    assert!(!true == false);
    assert_eq!(!5i32, -6);
    assert_eq!(!0u8, 255);

    // switch with several values per case, no fallthrough
    let size = |n: i32| match n {
        1 | 2 => "small",
        3 => "three",
        _ => "other",
    };
    assert_eq!(
        (size(1), size(2), size(3), size(9)),
        ("small", "small", "three", "other")
    );

    // break inside a match leaves the loop
    let mut seen = Vec::new();
    for n in 0..6 {
        match n {
            3 => break,
            _ => seen.push(n),
        }
    }
    assert_eq!(seen, [0, 1, 2]);

    // while let instead of while (x = next())
    let mut stack = vec![1, 2, 3];
    let mut popped = Vec::new();
    while let Some(top) = stack.pop() {
        popped.push(top);
    }
    assert_eq!(popped, [3, 2, 1]);

    // goto: a labeled break leaves two loops
    let mut hit = (0, 0);
    'outer: for r in 0..3 {
        for c in 0..3 {
            if r * c == 2 {
                hit = (r, c);
                break 'outer;
            }
        }
    }
    assert_eq!(hit, (1, 2));

    // a loop that gives a value with break
    let mut n = 0;
    let first_square_over_20 = loop {
        n += 1;
        if n * n > 20 {
            break n * n;
        }
    };
    assert_eq!(first_square_over_20, 25);

    // exceptions: Result and ?, panic! for bugs; catch_unwind exists but is not a try/catch
    assert_eq!(parse("41"), Ok(42));
    assert!(parse("x").is_err());
    std::panic::set_hook(Box::new(|_| {}));
    let caught = std::panic::catch_unwind(|| {
        let v: Vec<i32> = Vec::new();
        v[0]
    });
    assert!(caught.is_err());

    // default arguments: a struct with Default, or an Option parameter
    #[derive(Default)]
    struct Opts {
        width: u32,
        border: bool,
    }
    let o = Opts {
        width: 3,
        ..Default::default()
    };
    assert_eq!((o.width, o.border), (3, false));
    assert_eq!(greet("Ada", None), "Hello, Ada!");
    assert_eq!(greet("Ada", Some("Hi")), "Hi, Ada!");

    // constructors: a struct literal, or a plain function called new
    let p = Point { x: 1, y: 2 };
    let q = Point::new(1, 2);
    assert_eq!((p.x, p.y), (q.x, q.y));

    // inheritance: shared behaviour from a trait, with a default method
    assert_eq!(Dog.hello(), "hi Rex");

    // strings: no s[0]
    let s = String::from("héllo");
    assert_eq!(s.chars().next(), Some('h'));
    assert_eq!(s.as_bytes()[0], b'h');
    assert_eq!(s.chars().nth(1), Some('é'));

    // a[i++] as a block expression
    let a = [10, 20, 30];
    let mut i = 0;
    let v = {
        let t = a[i];
        i += 1;
        t
    };
    assert_eq!((v, i), (10, 1));
    let w = a[{
        i += 1;
        i - 1
    }];
    assert_eq!((w, i), (20, 2));

    // loops: while, loop, for over a slice, enumerate
    let mut n = 3;
    while n > 0 {
        n -= 1;
    }
    assert_eq!(n, 0);
    let xs = vec![5, 6];
    let mut seen = Vec::new();
    for v in &xs {
        seen.push(*v);
    }
    for (i, v) in xs.iter().enumerate() {
        seen.push(i as i32 * 100 + *v);
    }
    assert_eq!(seen, [5, 6, 5, 106]);

    // a lookup that may fail
    let mut m = HashMap::new();
    m.insert("a", 1);
    if let Some(v) = m.get("a") {
        assert_eq!(*v, 1);
    }
    assert!(m.get("zz").is_none());

    // a switch without a subject: a match with guards
    let tag = |x: i32| match x {
        _ if x > 5 => "big",
        _ => "small",
    };
    assert_eq!((tag(9), tag(1)), ("big", "small"));

    // swapping without a temporary, and struct update
    let (mut p, mut q) = (1, 2);
    (p, q) = (q, p);
    assert_eq!((p, q), (2, 1));
    #[derive(Clone, Copy)]
    struct Pt {
        x: i32,
        y: i32,
    }
    let base = Pt { x: 1, y: 2 };
    let moved = Pt { x: 5, ..base };
    assert_eq!((moved.x, moved.y), (5, 2));

    // Kotlin's ?. ?: !! and `?: return`
    fn len_of(name: &Option<String>) -> Option<usize> {
        name.as_ref().map(|s| s.len())
    }
    fn first_or_minus(v: &[i32]) -> i32 {
        let Some(f) = v.first() else { return -1 };
        *f
    }
    assert_eq!(len_of(&Some("abc".to_string())), Some(3));
    assert_eq!(len_of(&None), None);
    assert_eq!(first_or_minus(&[]), -1);
    assert_eq!(first_or_minus(&[7]), 7);
    let anon: Option<&str> = None;
    assert_eq!(anon.unwrap_or("anon"), "anon");
    assert_eq!(Some(2).unwrap(), 2);

    // Python's // ** chained comparison in is
    assert_eq!(7 / 2, 3);
    assert_eq!(-7 / 2, -3);
    assert_eq!(2_i32.pow(10), 1024);
    let x = 3;
    assert!(1 < x && x < 5);
    assert!([1, 2, 3].contains(&x));
    assert!(None::<i32>.is_none());

    // C: char arithmetic, casts, a pointer-like Option
    assert_eq!('7'.to_digit(10), Some(7));
    assert_eq!(3 as f64, 3.0);
    let n3 = 3;
    assert_eq!(n3 as f64, 3.0);
    let boxed = Box::new(Pt { x: 1, y: 2 });
    assert_eq!(boxed.x, 1);

    // C++: a generic function with a bound
    fn largest<T: PartialOrd>(a: T, b: T) -> T {
        if a > b { a } else { b }
    }
    assert_eq!(largest(2, 3), 3);
    assert_eq!(largest("a", "b"), "b");

    // C#: TryParse and the null-conditional chain
    assert_eq!("12".parse::<i32>().ok(), Some(12));
    assert_eq!("x".parse::<i32>().ok(), None);
    let a5: Option<i32> = None;
    assert_eq!(a5.unwrap_or(5), 5);

    // TypeScript: an optional parameter and a closure
    fn scale(x: f64, factor: Option<f64>) -> f64 {
        x * factor.unwrap_or(1.0)
    }
    assert_eq!(scale(2.0, None), 2.0);
    let inc = |x: f64| x + 1.0;
    assert_eq!(inc(1.0), 2.0);
    let d = 1.5;
    assert_eq!(d * 2.0, 3.0);

    // chars().next() is an Option<char>
    let first: Option<char> = "héllo".chars().next();
    assert_eq!(first, Some('h'));

    // the rustc suggestion for `null` is a raw pointer, not an Option
    let raw: *const i32 = std::ptr::null();
    assert!(raw.is_null());

    // a lookup that may fail is an Option
    let mut ages = HashMap::new();
    ages.insert("ada", 36);
    assert_eq!(ages.get("bob"), None);
}
