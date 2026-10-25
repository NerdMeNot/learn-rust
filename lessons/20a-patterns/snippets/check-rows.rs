// Backs the Rust cells in the tables on the page and a few one-line claims. Never shown.
use std::collections::HashMap;
use std::convert::Infallible;

struct Point {
    x: i32,
    y: i32,
}

enum Shape {
    Circle(f64),
    Square,
}

fn main() {
    // let Point { x, y } = p;  and  let Point { x: px, .. } = p;
    let p = Point { x: 3, y: -2 };
    let Point { x, y } = p;
    assert_eq!((x, y), (3, -2));
    let Point { x: px, .. } = p;
    assert_eq!(px, 3);

    // let Point { x, y } = &p;  gives references
    let Point { x, y } = &p;
    let (rx, ry): (&i32, &i32) = (x, y);
    assert_eq!((*rx, *ry), (3, -2));

    // let Point { x, y } = &mut p;  gives mutable references: write *x
    let mut q = Point { x: 3, y: -2 };
    let Point { x, y } = &mut q;
    *x += 10;
    *y += 1;
    assert_eq!((q.x, q.y), (13, -1));

    // a function parameter is a pattern
    fn manhattan(Point { x, y }: &Point) -> i32 {
        x.abs() + y.abs()
    }
    assert_eq!(manhattan(&p), 5);

    // (a, b) = (b, a);
    let (mut a, mut b) = (1, 2);
    (a, b) = (b, a);
    assert_eq!((a, b), (2, 1));

    // for (i, v) in xs.iter().enumerate()
    let xs = [10, 20];
    let pairs: Vec<(usize, &i32)> = xs.iter().enumerate().collect();
    assert_eq!(pairs, [(0, &10), (1, &20)]);

    // for (i, c) in s.char_indices()
    let indices: Vec<(usize, char)> = "aé!".char_indices().collect();
    assert_eq!(indices, [(0, 'a'), (1, 'é'), (3, '!')]);

    // let Some((key, value)) = s.split_once('=') else { .. };
    assert_eq!("mode=fast".split_once('='), Some(("mode", "fast")));
    assert_eq!("mode".split_once('='), None);
    let Some((key, value)) = "mode=fast".split_once('=') else {
        panic!("has an '='")
    };
    assert_eq!((key, value), ("mode", "fast"));

    // if let Some(v) = m.get(&k)  and  for (k, v) in &m
    let ages = HashMap::from([("ana", 30)]);
    assert!(matches!(ages.get("ana"), Some(30)));
    assert!(ages.get("zed").is_none());
    for (k, v) in &ages {
        assert_eq!((*k, *v), ("ana", 30));
    }

    // if let Shape::Circle(r) = o  and  let Shape::Circle(r) = o else { return };
    fn radius(o: &Shape) -> Option<f64> {
        let Shape::Circle(r) = o else {
            return None;
        };
        Some(*r)
    }
    assert_eq!(radius(&Shape::Circle(2.0)), Some(2.0));
    assert_eq!(radius(&Shape::Square), None);

    // if let [first, rest @ ..] = xs.as_slice()
    let list = vec![1, 2, 3];
    let [first, rest @ ..] = list.as_slice() else {
        panic!("not empty")
    };
    assert_eq!((*first, rest), (1, &[2, 3][..]));

    // let &x = r;  is the pattern form of  x = *r;  and  ref x  borrows
    let five = 5;
    let &v = &five;
    assert_eq!(v, 5);
    let name = Some(String::from("ana"));
    if let Some(ref s) = name {
        assert_eq!(s, "ana");
    }

    // an error type with no values cannot fail: Ok(x) always fits
    let sure: Result<u32, Infallible> = Ok(7);
    let Ok(n) = sure;
    assert_eq!(n, 7);

    println!("all rows hold");
}
