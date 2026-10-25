// Backs the pattern table and the "your language, in Rust" rows. Never shown.
#![allow(clippy::redundant_guards)]

fn main() {
    // literal, range, exclusive range, open ranges
    assert!(matches!(5, 5));
    assert!(matches!('m', 'a'..='z'));
    assert!(matches!(5, 1..=5));
    assert!(!matches!(5, 1..5));
    assert!(matches!(4, 1..5));
    assert!(matches!(99, 10..));
    assert!(!matches!(9, 10..));
    for n in [-3, 0] {
        assert!(match n {
            ..=0 => true,
            _ => false,
        });
    }

    // `..0` stops before 0; `101..` is 101 or more; the Go and Kotlin "greater than" rows
    assert!(matches!(-1, ..0));
    assert!(!matches!(0, ..0));
    assert!(matches!(101, 101..));
    let size = |n: i32| match n {
        ..0 => "negative",
        0 => "zero",
        1..=100 => "some",
        101.. => "big",
    };
    assert_eq!(size(500), "big");

    // alternatives
    assert!(matches!(2, 1 | 2));
    assert!(matches!("b", "a" | "b"));

    // `_` ignores, a name binds
    let got = match 7 {
        _ => "ignored",
    };
    assert_eq!(got, "ignored");
    let doubled = match 7 {
        n => n * 2,
    };
    assert_eq!(doubled, 14);

    // @ binds and tests
    let label = match 4 {
        n @ 1..=5 => n * 10,
        _ => 0,
    };
    assert_eq!(label, 40);

    // Python's `as`: bind the whole thing and test it
    assert!(matches!(2, k @ (1 | 2 | 3) if k == 2));

    // tuples and nesting
    assert!(matches!((0, 5), (0, _)));
    assert!(matches!(((1, 2), 3), ((_, 2), 3)));

    // slices
    let v = [1, 2, 3, 4];
    assert!(matches!(&v[..], [1, .., 4]));
    assert!(matches!(&v[..], [1, ..]));
    assert!(matches!(&v[..], [_, _, _, _]));
    assert!(!matches!(&v[..], [_, _]));
    let (head, rest_len) = match &v[..] {
        [head, rest @ ..] => (*head, rest.len()),
        [] => (0, 0),
    };
    assert_eq!((head, rest_len), (1, 3));
    assert!(matches!(&v[1..], [_, _, last] if *last == 4));
    // an empty slice matches [] and no other length
    let empty: &[i32] = &[];
    assert!(matches!(empty, []));

    // arrays take the same patterns
    let arr = [1, 2, 3];
    assert!(matches!(arr, [1, .., 3]));
    let [a, b, c] = arr;
    assert_eq!(a + b + c, 6);

    // struct and Option patterns
    struct P {
        x: i32,
        y: i32,
    }
    let p = P { x: 0, y: 9 };
    assert!(matches!(p, P { x: 0, .. }));
    assert!(matches!(p, P { y, .. } if y > 5));
    assert!(matches!(Some(3), Some(1..=9)));
    assert!(matches!(Some(30), Some(n) if n > 9));
    assert!(matches!(None::<i32>, None));

    // a type test with a name: an enum variant, in `match` and in `if let`
    enum Shape {
        Circle(i32),
        Rect { w: i32 },
    }
    let shape = Shape::Circle(2);
    assert!(matches!(shape, Shape::Circle(2)));
    if let Shape::Circle(c) = shape {
        assert_eq!(c, 2);
    }
    let wide = Shape::Rect { w: 9 };
    assert!(matches!(wide, Shape::Rect { w: 5.., .. }));

    // Python's dict pattern has no match equivalent: look the key up
    let mut modes = std::collections::HashMap::new();
    modes.insert("mode", "fast");
    assert!(matches!(modes.get("mode"), Some(&"fast")));
    if let Some(m) = modes.get("mode") {
        assert_eq!(*m, "fast");
    }

    // patterns also follow `let`, sit in parameters, and follow `for`
    let (pa, pb) = (1, 2);
    let pair_sum = |(a, b): (i32, i32)| a + b;
    assert_eq!(pair_sum((pa, pb)), 3);
    for (i, c) in "ab".chars().enumerate() {
        assert!(matches!((i, c), (0, 'a') | (1, 'b')));
    }

    // a reference pattern takes the & off; a literal pattern also sees through one
    let r = &5;
    assert!(matches!(r, &5));
    assert!(matches!(r, 5));
    let &copy = r;
    assert_eq!(copy, 5);

    // guards: first arm that fits wins, `_` last
    let describe = |n: i32| match n {
        n if n < 0 => "negative",
        0 => "zero",
        _ => "positive",
    };
    assert_eq!(describe(-4), "negative");
    assert_eq!(describe(0), "zero");
    assert_eq!(describe(9), "positive");

    // a guard after `|` covers all the alternatives
    let which = |n: i32, ok: bool| match n {
        1 | 2 if ok => "listed and ok",
        1 | 2 => "listed",
        _ => "other",
    };
    assert_eq!(which(2, true), "listed and ok");
    assert_eq!(which(2, false), "listed");

    // a `switch` with no subject (Go, TypeScript `switch (true)`) is guards
    let band = |n: i32| match () {
        _ if n < 10 => "small",
        _ if n < 100 => "medium",
        _ => "large",
    };
    assert_eq!(band(5), "small");
    assert_eq!(band(50), "medium");
    assert_eq!(band(500), "large");

    // a Python/Java/Kotlin "else" or "default" is `_`; it must come last
    let late = |n: i32| match n {
        1 => "one",
        _ => "default",
    };
    assert_eq!(late(1), "one");
    assert_eq!(late(5), "default");

    // char, &str and String
    assert_eq!(
        match 'q' {
            'a'..='m' => 1,
            'n'..='z' => 2,
            _ => 0,
        },
        2
    );
    let owned = String::from("go");
    assert!(matches!(owned.as_str(), "go" | "run"));

    // exhaustive without `_`: bool, u8 ranges
    let b = true;
    assert_eq!(
        match b {
            true => 1,
            false => 0,
        },
        1
    );
    let x: u8 = 255;
    assert_eq!(
        match x {
            0..=127 => "low",
            128..=255 => "high",
        },
        "high"
    );

    // an arm that never returns has type `!` and fits any other arm
    let n: i32 = match "12".parse() {
        Ok(n) => n,
        Err(_) => panic!("not a number"),
    };
    assert_eq!(n, 12);

    println!("all checks passed");
}
