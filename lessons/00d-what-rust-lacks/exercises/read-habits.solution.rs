// Read this program. Replace each `todo!()` with what the marked call gives, then run
// until the checks print `all checks passed`.
fn tally() -> i32 {
    let mut total = 0;
    for i in 0..6 {
        match i {
            1 | 2 => total += 10,
            3 => break,
            _ => total += 1,
        }
    }
    total
}

fn halves() -> (i32, i32) {
    (-7 / 2, 7 / 2)
}

fn city_len(city: Option<&str>) -> Option<usize> {
    let c = city?;
    Some(c.len())
}

/// Hint: how many times does the loop body run, and what does `break` leave?
fn tally_is() -> i32 {
    21
}

/// Hint: the value of `halves()`: integer division with a negative number
fn halves_are() -> (i32, i32) {
    (-3, 3)
}

/// Hint: what does `city_len(None)` give? Look at what `?` does
fn no_city_gives() -> Option<usize> {
    None
}

/// Hint: what does `city_len(Some(""))` give? Is an empty string a missing value?
fn empty_city_gives() -> Option<usize> {
    Some(0)
}

/// Hint: what does `city_len(Some("Oslo"))` give?
fn oslo_gives() -> Option<usize> {
    Some(4)
}

fn check<T: PartialEq + std::fmt::Debug>(call: &str, said: T, truth: T) {
    assert!(said == truth, "misread `{call}`: you said {said:?}");
    println!("ok: {call} = {said:?}");
}

fn main() {
    check("tally()", tally_is(), tally());
    check("halves()", halves_are(), halves());
    check("city_len(None)", no_city_gives(), city_len(None));
    check(
        "city_len(Some(\"\"))",
        empty_city_gives(),
        city_len(Some("")),
    );
    check(
        "city_len(Some(\"Oslo\"))",
        oslo_gives(),
        city_len(Some("Oslo")),
    );
    println!("all checks passed");
}
