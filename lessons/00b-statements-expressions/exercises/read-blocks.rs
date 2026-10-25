// Read this program without running it. Then replace each `todo!()` below with the line
// that `show` makes for that variable, written as a string, in the form "x = 1".

fn double(x: i32) -> i32 {
    x * 2
}

fn show(n: i32) -> Vec<String> {
    let a = { double(n) };
    let b = {
        double(n);
    };
    let c = if n > 5 { n } else { n * 10 };
    let d = {
        let m = double(n);
        m * 10 + 1
    };
    vec![
        format!("a = {a:?}"),
        format!("b = {b:?}"),
        format!("c = {c:?}"),
        format!("d = {d:?}"),
    ]
}

// What `show` makes:
/// Hint: the last line of the block has no `;`, so it is the block's value
fn a_when_n_is_3() -> &'static str {
    todo!("the last line of the block has no `;`, so it is the block's value")
}

/// Hint: look at the end of the last line in the block: what does a `;` make of the value?
fn b_when_n_is_3() -> &'static str {
    todo!("look at the end of the last line in the block: what does a `;` make of the value?")
}

/// Hint: which branch of the `if` runs when n is 3?
fn c_when_n_is_3() -> &'static str {
    todo!("which branch of the `if` runs when n is 3?")
}

/// Hint: same `if`, the other branch: 7 is not below 5
fn c_when_n_is_7() -> &'static str {
    todo!("same `if`, the other branch: 7 is not below 5")
}

/// Hint: a block can hold a `let` first; its value is still the last line
fn d_when_n_is_3() -> &'static str {
    todo!("a block can hold a `let` first; its value is still the last line")
}

fn check(what: &str, said: &str, truth: &str) {
    assert!(said == truth, "misread `{what}`: you said {said:?}");
    println!("ok: {what} is {said:?}");
}

fn main() {
    let at3 = show(3);
    let at7 = show(7);
    check("a with n = 3", a_when_n_is_3(), &at3[0]);
    check("b with n = 3", b_when_n_is_3(), &at3[1]);
    check("c with n = 3", c_when_n_is_3(), &at3[2]);
    check("c with n = 7", c_when_n_is_7(), &at7[2]);
    check("d with n = 3", d_when_n_is_3(), &at3[3]);
    println!("all checks passed");
}
