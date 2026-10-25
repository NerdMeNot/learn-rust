// Read the format strings and attributes, then write what each line produces, spaces
// included. Replace the `todo!()` and run until the checks print `all checks passed`.
//
// `#[cfg(test)]` keeps the next item only in a test build; `#[cfg(not(test))]` is the
// opposite and keeps it when `test` is off, which is the case in a normal run.
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
fn build() -> &'static str {
    "test"
}

#[cfg(not(test))]
fn build() -> &'static str {
    "normal"
}

fn shown() -> [String; 6] {
    let p = Point { x: 3, y: -4 };
    let q = p.clone();
    [
        format!("{:5}|{:5}|", 42, "ab"),
        format!("{:05}|{:+}|{:#x}", 42, 42, 255),
        format!("{:.2}|{:8.3}|", 3.14159, 3.14159),
        format!("{{}}{}", 7),
        format!("{p:?}"),
        format!("{} {}", p == q, build()),
    ]
}

/// Hint, one per line:
/// 1. a number sits at the right of its width, text at the left.
/// 2. `0` pads with zeros, `+` shows the sign, `#x` is hex with a `0x` prefix.
/// 3. `.2` is two decimals; `8.3` is a width of 8 and three decimals.
/// 4. a doubled brace is one brace.
/// 5. `:?` is the Debug form, which a derive writes as `Name { field: value }`.
/// 6. `==` gives `true` or `false`; `build()` is the `cfg(not(test))` one in a normal run.
fn answers() -> [&'static str; 6] {
    [
        "   42|ab   |",
        "00042|+42|0xff",
        "3.14|   3.142|",
        "{}7",
        "Point { x: 3, y: -4 }",
        "true normal",
    ]
}

fn main() {
    let (shown, answers) = (shown(), answers());
    for i in 0..6 {
        assert!(
            answers[i] == shown[i],
            "line {} of `shown`: you wrote {:?}",
            i + 1,
            answers[i]
        );
        println!("ok: line {} is {:?}", i + 1, answers[i]);
    }
    println!("all checks passed");
}
