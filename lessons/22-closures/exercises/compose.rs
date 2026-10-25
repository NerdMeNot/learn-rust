// Three functions that return a closure. Each returned closure must keep working after
// the function that made it has returned, so it has to own what it uses.

// A closure that adds `n` to its argument.
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |_x| todo!("return a closure that adds `n`; it uses `n` after this function is gone")
}

// A closure that returns a running total: each call adds `step` and returns the total.
// Every counter made by this function has its own total.
fn make_counter(step: i32) -> impl FnMut() -> i32 {
    || todo!("return a closure with its own `total`; each call adds `step` and returns it")
}

// A closure that applies `first` to its argument and then `second` to the result.
fn compose(first: impl Fn(i32) -> i32, second: impl Fn(i32) -> i32) -> impl Fn(i32) -> i32 {
    |_x| todo!("return a closure that calls both; it must own `first` and `second`")
}

fn main() {
    let add2 = make_adder(2);
    let add10 = make_adder(10);
    assert_eq!(add2(1), 3);
    assert_eq!(add10(1), 11);

    // Two counters made by the same function do not share a total, even when their
    // calls are interleaved.
    let mut by_one = make_counter(1);
    let mut by_five = make_counter(5);
    assert_eq!(by_one(), 1);
    assert_eq!(by_one(), 2);
    assert_eq!(by_five(), 5);
    assert_eq!(by_one(), 3);
    assert_eq!(by_five(), 10);

    let add_then_double = compose(make_adder(1), |x| x * 2);
    let double_then_add = compose(|x| x * 2, make_adder(1));
    assert_eq!(add_then_double(5), 12);
    assert_eq!(double_then_add(5), 11);

    // A composed closure can be composed again.
    let add3 = compose(make_adder(1), make_adder(2));
    let add_then_double_then_negate = compose(add_then_double, |x| -x);
    assert_eq!(add3(0), 3);
    assert_eq!(add_then_double_then_negate(5), -12);
    println!("all checks passed");
}
