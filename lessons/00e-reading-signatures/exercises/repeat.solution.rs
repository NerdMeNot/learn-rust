// Call `f` exactly `n` times. The bound says `f` may change what it captured.
fn repeat(n: usize, mut f: impl FnMut()) {
    for _ in 0..n {
        f();
    }
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
