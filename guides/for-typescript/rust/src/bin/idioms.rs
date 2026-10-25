use std::collections::HashMap;
use std::fmt::Debug;

// One line per row of the page's table: row number, then the value.
fn row<T: Debug>(n: u32, v: T) {
    println!("{n:>2} {v:?}");
}

struct Point {
    x: i32,
    y: i32,
}

struct User {
    name: String,
}

fn main() {
    let n = 3;
    let x: Option<i32> = None;
    let u: Option<User> = Some(User { name: "ana".to_string() });
    let xs = vec![1, 2, 3, 4];
    let ys = vec![5, 6];
    let s = "héllo🙂";
    let num = "12";
    let mut m = HashMap::new();
    let p = Point { x: 1, y: 2 };

    row(1, format!("{n} items"));
    row(2, x.unwrap_or(5));
    row(3, u.as_ref().map(|u| &u.name));
    row(4, xs.iter().map(|x| x * 2).collect::<Vec<_>>());
    row(5, xs.iter().copied().filter(|x| x % 2 == 0).collect::<Vec<_>>());
    row(6, xs.iter().fold(0, |a, b| a + b));
    row(7, xs.iter().find(|&&x| x > 2));
    row(8, xs.contains(&3));
    row(9, &xs[1..3]);
    row(10, [xs.as_slice(), ys.as_slice()].concat());
    let mut v = vec![10, 9, 1];
    v.sort();
    row(11, &v);
    let a = vec![1, 2];
    let b = vec![1, 2];
    row(12, a == b);
    row(13, (s.len(), s.chars().count()));
    m.insert("a", 1);
    row(14, m.get("b").copied().unwrap_or(0));
    let mut pairs = Vec::new();
    for (i, x) in xs.iter().enumerate() {
        pairs.push((i, *x));
    }
    row(15, pairs);
    row(16, (num.parse::<i32>(), "x".parse::<i32>()));
    let Point { x: px, y: py } = p;
    row(17, (px, py));
    let double = |x: i32| x * 2;
    row(18, double(21));
    row(19, (0..3).map(|i| i * i).collect::<Vec<_>>());
    row(20, xs.iter().all(|&x| x > 0));
}
