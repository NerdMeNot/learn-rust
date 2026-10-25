fn double(n: i32) -> i32 {
    n * 2
}

fn negate(n: i32) -> i32 {
    -n
}

fn identity(n: i32) -> i32 {
    n
}

/// The function called `name`: "double", "negate", or `identity` for anything else.
fn by_name(name: &str) -> fn(i32) -> i32 {
    match name {
        "double" => double,
        "negate" => negate,
        _ => identity,
    }
}

/// Apply `f` to `x`, then to the result, `times` times in all. `f` may capture variables.
fn apply_n(f: impl Fn(i32) -> i32, times: u32, x: i32) -> i32 {
    let mut value = x;
    for _ in 0..times {
        value = f(value);
    }
    value
}

fn main() {
    assert_eq!(by_name("double")(4), 8);
    assert_eq!(by_name("negate")(4), -4);
    assert_eq!(by_name("nope")(4), 4);

    assert_eq!(apply_n(double, 3, 1), 8);
    assert_eq!(apply_n(negate, 2, 7), 7);
    // Zero times means the input comes back unchanged.
    assert_eq!(apply_n(double, 0, 5), 5);
    // A closure that captures `step` works because the parameter is `impl Fn`.
    let step = 7;
    assert_eq!(apply_n(|n| n + step, 3, 0), 21);

    println!("all checks passed");
}
