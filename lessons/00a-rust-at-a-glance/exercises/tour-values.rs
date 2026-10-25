// Read this program without running it. Then replace each `todo!()` below with what
// the call gives, written as you would write it in code. An `Err` here holds a `String`,
// so a bare text needs `.to_string()`: `Err("oops".to_string())`.

fn label(n: i32) -> String {
    match n {
        0 => String::from("zero"),
        1..=9 => format!("small {n}"),
        big @ 10..=99 => format!("big {big}"),
        _ => String::from("huge"),
    }
}

fn halve(n: i32) -> Result<i32, String> {
    if n % 2 != 0 {
        return Err(format!("{n} is odd"));
    }
    Ok(n / 2)
}

fn quarter(n: i32) -> Result<i32, String> {
    let half = halve(n)?;
    halve(half)
}

// What each call gives:

/// Hint: which arm does 9 fall in, and what text does that arm build?
fn label_of_9() -> &'static str {
    todo!("which arm does 9 fall in, and what text does that arm build?")
}

/// Hint: `big @ 10..=99` names the number it matched
fn label_of_10() -> &'static str {
    todo!("`big @ 10..=99` names the number it matched")
}

/// Hint: is 100 inside `10..=99`? `..=` includes the end, and nothing past it
fn label_of_100() -> &'static str {
    todo!("is 100 inside `10..=99`? `..=` includes the end, and nothing past it")
}

/// Hint: both calls to `halve` succeed: 12, then 6, then 3
fn quarter_of_12() -> Result<i32, String> {
    todo!("both calls to `halve` succeed: 12, then 6, then 3")
}

/// Hint: the first `halve` works; which number does the second one reject?
fn quarter_of_6() -> Result<i32, String> {
    todo!("the first `halve` works; which number does the second one reject?")
}

/// Hint: what does `?` do when `halve(n)` is an `Err`?
fn quarter_of_7() -> Result<i32, String> {
    todo!("what does `?` do when `halve(n)` is an `Err`?")
}

fn check<T: PartialEq + std::fmt::Debug>(call: &str, said: T, truth: T) {
    assert!(said == truth, "misread `{call}`: you said {said:?}");
    println!("ok: {call} = {said:?}");
}

fn main() {
    check("label(9)", label_of_9(), label(9).as_str());
    check("label(10)", label_of_10(), label(10).as_str());
    check("label(100)", label_of_100(), label(100).as_str());
    check("quarter(12)", quarter_of_12(), quarter(12));
    check("quarter(6)", quarter_of_6(), quarter(6));
    check("quarter(7)", quarter_of_7(), quarter(7));
    println!("all checks passed");
}
