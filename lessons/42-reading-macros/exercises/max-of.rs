use std::cell::Cell;

// max_of!(a, b, c, ...) is the largest of one or more values, written as a macro because a
// function cannot take any number of arguments.
macro_rules! max_of {
    ($x:expr $(,)?) => {
        $x
    };
    ($head:expr, $($tail:expr),+ $(,)?) => {{
        let first = $head;
        // The starter hands back `first` so the types work out; the rest is yours.
        if false {
            first
        } else {
            todo!("compare first with the maximum of the rest, evaluating each argument once")
        }
    }};
}

fn main() {
    assert_eq!(max_of!(7), 7);
    assert_eq!(max_of!(3, 9, 4), 9);
    assert_eq!(max_of!(-5, -2), -2);
    assert_eq!(max_of!(1, 2 + 3 * 4, 5), 14);
    assert_eq!(max_of!("pear", "apple", "fig"), "pear");
    assert_eq!(max_of!(1.5, 0.5,), 1.5); // a trailing comma is allowed

    // Each argument is evaluated exactly once, left to right.
    let calls = Cell::new(0);
    let next = || {
        calls.set(calls.get() + 1);
        calls.get()
    };
    let largest = max_of!(next(), next(), next());
    assert_eq!(calls.get(), 3, "each argument must be evaluated once");
    assert_eq!(largest, 3);
    println!("all checks passed");
}
