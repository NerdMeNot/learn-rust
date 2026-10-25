// Call `f` exactly `n` times. The bound says `f` may change what it captured.
fn repeat(n: usize, f: impl FnMut()) {
    todo!(
        "loop `n` times and call `f()`; the compiler will ask for one more word in the parameter list"
    )
}

fn main() {
    let mut count = 0;
    repeat(3, || count += 1);
    assert_eq!(count, 3);

    let mut log = Vec::new();
    repeat(2, || log.push("tick"));
    assert_eq!(log, ["tick", "tick"]);

    repeat(0, || panic!("must not be called"));
    println!("all checks passed");
}
