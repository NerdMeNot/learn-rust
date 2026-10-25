// Backs the "in Rust" cells of the tables in lesson.mdx. Never shown.
use std::collections::HashMap;

const MAX: u32 = 100;
const MAX_LEVEL: u32 = 99;

struct P {
    x: i32,
    y: i32,
}

struct Stack<T> {
    items: Vec<T>,
}

#[allow(dead_code)]
enum Shape {
    Circle(f64),
    Rect { w: f64, h: f64 },
    Empty,
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn ident<T>(x: T) -> T {
    x
}

fn main() {
    let x = 5;
    let mut y = 5;
    y += x;
    let typed: i32 = 5;
    let later: i32;
    later = 5;
    assert_eq!((x, y, typed, later), (5, 10, 5, 5));
    assert_eq!((MAX, MAX_LEVEL), (100, 99));
    assert_eq!(add(1, 2), 3);

    let (a, b) = (1, "two");
    assert_eq!((a, b), (1, "two"));
    let arr: [i32; 3] = [1, 2, 3];
    let v: Vec<i32> = vec![1, 2, 3];
    assert_eq!(arr, [1, 2, 3]);
    assert_eq!(v, [1, 2, 3]);
    let p = P { x: 1, y: 2 };
    let P { x: px, y: py } = p;
    assert_eq!(px + py, 3);
    let s: Stack<i32> = Stack { items: vec![] };
    assert!(s.items.is_empty());
    assert_eq!(ident::<i32>(5), 5);
    let m = HashMap::<String, i32>::new();
    assert!(m.is_empty());
    assert!(Vec::<String>::new().is_empty());

    let shape = Shape::Rect { w: 2.0, h: 3.0 };
    assert!(matches!(shape, Shape::Rect { w, .. } if w == 2.0));
    assert!(matches!(Shape::Circle(1.0), Shape::Circle(_)) && matches!(Shape::Empty, Shape::Empty));
    let narrow: i32 = 7;
    let widened: i64 = i64::from(narrow); // no implicit widening: it is written out
    assert_eq!(widened, 7);
    assert!(u8::try_from(300i32).is_err());
    assert_eq!((b'a' as char, 'a' as u32, true as i32), ('a', 97, 1));
    assert_eq!(0_usize.checked_sub(1), None);
    assert_eq!(std::cmp::max::<i32>(1, 2), 2);

    let inc = |x: i32| x + 1;
    let inc2 = |x| x + 1;
    assert_eq!((inc(1), inc2(1)), (2, 2));

    let n = 7;
    let price = 2.9f64;
    assert_eq!((n as f64, price as i32, -price as i32), (7.0, 2, -2));

    assert_eq!('R' as u32, 82);
    assert_eq!(b'R', 82u8);
    assert_eq!(r"C:\temp".len(), 7);
    assert_eq!("42".parse::<i32>(), Ok(42));
    assert_eq!((0xff, 1_000, 0b101, 0o17, 0o377), (255, 1000, 5, 15, 255));
    assert_eq!((017, 0377), (17, 377)); // a leading zero is NOT octal in Rust
    assert_eq!((5u32, 5u64, 2.5f32, 5_000i64), (5, 5, 2.5, 5000));
    assert_eq!(size_of::<char>(), 4);
    assert_eq!(0..5, std::ops::Range { start: 0, end: 5 });
    let xs = [1, 2, 3, 4, 5];
    assert_eq!(&xs[1..4], [2, 3, 4]);
    assert_eq!(&xs[1..=3], [2, 3, 4]);
    assert_eq!(&xs[..2], [1, 2]);
    assert_eq!(&xs[3..], [4, 5]);
    let [first, .., last] = xs;
    assert_eq!((first, last), (1, 5));
}
