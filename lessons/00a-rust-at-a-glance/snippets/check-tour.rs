// Backs the claims in the token tables of lesson.mdx. Never shown.
fn bucket(n: u32) -> &'static str {
    match n {
        0 => "zero",
        1..=3 => "one to three",
        _ => "more",
    }
}

fn exclusive(n: u32) -> &'static str {
    match n {
        0..3 => "below three",
        _ => "three or more",
    }
}

fn keep(n: u32) -> u32 {
    match n {
        0 => 100,
        other => other,
    }
}

fn first_number(text: &str) -> Result<i32, std::num::ParseIntError> {
    let n = text.trim().parse::<i32>()?;
    Ok(n)
}

fn parse_or_zero(text: &str) -> i32 {
    match text.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 0,
    }
}

struct Pair {
    words: usize,
    longest: String,
}

fn main() {
    // 1..=3 includes 3; 0..3 does not
    assert_eq!(bucket(3), "one to three");
    assert_eq!(bucket(4), "more");
    assert_eq!(exclusive(2), "below three");
    assert_eq!(exclusive(3), "three or more");
    // a bare name in an arm matches anything and holds the value
    assert_eq!(keep(0), 100);
    assert_eq!(keep(7), 7);
    // a tail expression is the value; a trailing `;` makes it ()
    let value = {
        let a = 2;
        a + 1
    };
    #[allow(unused_must_use)]
    let unit = {
        let a = 2;
        a + 1;
    };
    assert_eq!(value, 3);
    assert_eq!(unit, ());
    // field shorthand
    let words = 4;
    let longest = String::from("quick");
    let short = Pair { words, longest };
    let long = Pair {
        words: words,
        longest: String::from("quick"),
    };
    assert_eq!(short.words, long.words);
    assert_eq!(short.longest, long.longest);
    // ? returns the error early and unwraps the success
    assert_eq!(first_number(" 42 "), Ok(42));
    assert!(first_number("x").is_err());
    assert_eq!(parse_or_zero("x"), 0);
    // a closure is called like a function, and can be stored in a variable
    let double = |n: i32| n * 2;
    assert_eq!(double(4), 8);
    // :? is Debug, {} is Display
    assert_eq!(format!("{:?}", "hi"), "\"hi\"");
    assert_eq!(format!("{}", "hi"), "hi");
    // `.` reads a field through a reference too (no `->`)
    let pair = Pair {
        words: 1,
        longest: String::from("x"),
    };
    let by_ref = &pair;
    assert_eq!(by_ref.words, 1);
    assert_eq!(by_ref.longest.len(), 1);
    // `+=` changes a `mut` variable
    let mut total = 1;
    total += 1;
    assert_eq!(total, 2);
    println!("all token claims hold");
}
