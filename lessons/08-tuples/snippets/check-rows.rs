// Backs the "what you write today" tables: each Rust cell is compiled and run here.
use std::collections::HashMap;

fn pair() -> (i32, i32) {
    (3, 4)
}

fn triple() -> (i32, &'static str, bool) {
    (1, "two", true)
}

fn parse_both(a: &str, b: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    Ok((a.parse()?, b.parse()?))
}

fn main() {
    // Go: a, b := f()  /  _, b := f()  /  a, b = b, a
    let (a, b) = pair();
    assert_eq!((a, b), (3, 4));
    let (_, b2) = pair();
    assert_eq!(b2, 4);
    let (mut x, mut y) = (1, 2);
    (x, y) = (y, x);
    assert_eq!((x, y), (2, 1));

    // Go: fmt.Println(f()) prints both; Rust prints the tuple as one value.
    assert_eq!(format!("{:?}", pair()), "(3, 4)");
    // C++: std::tie(std::ignore, b) = f();
    let b3;
    (_, b3) = pair();
    assert_eq!(b3, 4);
    // Python / TS: a, b, c = b, c, a
    let (mut p1, mut p2, mut p3) = (1, 2, 3);
    (p1, p2, p3) = (p2, p3, p1);
    assert_eq!((p1, p2, p3), (2, 3, 1));
    // Early return needs the parentheses.
    fn early(flag: bool) -> (i32, i32) {
        if flag {
            return (1, 2);
        }
        (3, 4)
    }
    assert_eq!(early(true), (1, 2));

    // Go: (value, err) -> Result<(A, B), E>; (v, ok) -> Option<(A, B)>
    let found: Option<(&str, &str)> = "k=v".split_once('=');
    assert_eq!(found, Some(("k", "v")));
    assert_eq!(parse_both("3", "4"), Ok((3, 4)));
    assert!(parse_both("3", "x").is_err());

    // A tuple is a map key when its parts are.
    let mut grid: HashMap<(i32, i32), &str> = HashMap::new();
    grid.insert((2, -1), "flag");
    assert_eq!(grid.get(&(2, -1)), Some(&"flag"));
    assert_eq!(grid.get(&(9, 9)), None);

    // Go: v, ok := m[k]  ->  Option
    let mut m = HashMap::new();
    m.insert("a", 1);
    assert_eq!(m.get("a"), Some(&1));
    assert_eq!(m.get("z"), None);
    if let Some(v) = m.get("a") {
        assert_eq!(*v, 1);
    }

    // Kotlin: Pair / Triple / `to`
    let p = (3, 4);
    assert_eq!((p.0, p.1), (3, 4));
    let t = triple();
    assert_eq!(t.2, true);
    let kv = ("k", 1);
    assert_eq!(kv, ("k", 1));

    // Python: first, *rest = t has no tuple equivalent; the slice pattern does.
    let list = [1, 2, 3, 4];
    let [first, rest @ ..] = list;
    assert_eq!(first, 1);
    assert_eq!(rest, [2, 3, 4]);

    // Python: divmod(a, b) agrees with div_euclid / rem_euclid when b > 0.
    for a in -20i32..=20 {
        for b in 1i32..=7 {
            let (q, r) = (a.div_euclid(b), a.rem_euclid(b));
            assert_eq!(q * b + r, a);
            assert!((0..b).contains(&r));
            assert_eq!(q, (a as f64 / b as f64).floor() as i32);
        }
    }
    // ...and not when b < 0: Python's divmod(17, -5) is (-4, -3).
    assert_eq!((17i32.div_euclid(-5), 17i32.rem_euclid(-5)), (-3, 2));
    // Rust's `/` and `%` truncate toward zero, like C and Go.
    assert_eq!((-17 / 5, -17 % 5), (-3, -2));

    // Python / C++: tuples compare slot by slot.
    assert!((1, 10) > (1, 9));
    // One-slot tuple needs the comma.
    let one: (i32,) = (5,);
    assert_eq!(one.0, 5);

    // C: out pointers are `&mut` parameters; returning a tuple is the usual way.
    fn two(out_a: &mut i32, out_b: &mut i32) {
        *out_a = 1;
        *out_b = 2;
    }
    let (mut oa, mut ob) = (0, 0);
    two(&mut oa, &mut ob);
    assert_eq!((oa, ob), (1, 2));

    // Tuple elements can be arrays and nested tuples.
    let n = ((1, 2), [3, 4]);
    assert_eq!(n.0.1 + n.1[0], 5);

    // Tuple vs array: an array takes any index and loops; a tuple takes a literal slot.
    let arr = [10, 20, 30];
    let i = arr.len() - 1;
    assert_eq!(arr[i], 30);
    let mut total = 0;
    for x in arr {
        total += x;
    }
    assert_eq!(total, 60);

    println!("all rows hold");
}
