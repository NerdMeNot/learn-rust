// Read this program without running it. Then replace each `todo!()` below with the text
// that `show` makes for that variable (what `to_string()` gives).

fn show(input: &str) -> Vec<String> {
    let n = input.trim().len();
    let n = n as f64 / 4.0;
    let big = 1_000;
    let wrapped = big as u8;
    let byte = 0xff_u8;
    let signed = byte as i8;
    let raw = r"\n".len();
    let cooked = "\n".len();
    let half = "7".parse::<i32>().unwrap() / 2;
    let next = (b'a' + 1) as char;
    vec![
        n.to_string(),
        wrapped.to_string(),
        signed.to_string(),
        raw.to_string(),
        cooked.to_string(),
        half.to_string(),
        next.to_string(),
    ]
}

// What `show("  abcde ")` makes, in that order:
/// Hint: how long is `input` once `trim()` drops the spaces, and what does the second `let n` make of it?
fn n_is() -> &'static str {
    "1.25"
}

/// Hint: `as u8` keeps only the low 8 bits, which is the value modulo 256
fn wrapped_is() -> &'static str {
    "232"
}

/// Hint: `0xff` has all eight bits set; read as an `i8` (the top bit is the sign), what number is that?
fn signed_is() -> &'static str {
    "-1"
}

/// Hint: a raw string processes no escapes, so count the characters
fn raw_is() -> &'static str {
    "2"
}

/// Hint: in an ordinary string, how many characters is the escape for a newline?
fn cooked_is() -> &'static str {
    "1"
}

/// Hint: `parse::<i32>()` makes an integer, and integer `/` drops the fraction (7 / 2 is not 3.5)
fn half_is() -> &'static str {
    "3"
}

/// Hint: `b'a'` is a number; `as char` turns the sum back into a character
fn next_is() -> &'static str {
    "b"
}

fn check(variable: &str, said: &str, truth: &str) {
    assert!(said == truth, "misread `{variable}`: you said {said:?}");
    println!("ok: {variable} is {said:?}");
}

fn main() {
    let made = show("  abcde ");
    check("n", n_is(), &made[0]);
    check("wrapped", wrapped_is(), &made[1]);
    check("signed", signed_is(), &made[2]);
    check("raw", raw_is(), &made[3]);
    check("cooked", cooked_is(), &made[4]);
    check("half", half_is(), &made[5]);
    check("next", next_is(), &made[6]);
    println!("all checks passed");
}
