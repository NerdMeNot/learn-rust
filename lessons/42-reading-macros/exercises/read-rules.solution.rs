use std::cell::Cell;

// `rule!` answers with the number of the rule that matched. Read the patterns top to bottom.
macro_rules! rule {
    ($a:expr) => {
        1
    };
    ($a:expr, $b:expr) => {
        2
    };
    ($a:ident = $b:expr) => {
        3
    };
    ($($t:tt)*) => {
        4
    };
}

// `twice!(e)` pastes its argument in twice.
macro_rules! twice {
    ($e:expr) => {
        $e + $e
    };
}

// Part 1: which rule does each call match? Answer from the patterns, in this order:
// `2 + 3`, `1, 2`, `x = 5`, `a, b, c`.
fn which_rules() -> [u32; 4] {
    [1, 2, 1, 4]
}

// Part 2: write out by hand what `twice!(next())` becomes.
fn twice_by_hand(next: &dyn Fn() -> i32) -> i32 {
    next() + next()
}

fn main() {
    let actual = [rule!(2 + 3), rule!(1, 2), rule!(x = 5), rule!(a, b, c)];
    let answer = which_rules();
    for (i, call) in ["2 + 3", "1, 2", "x = 5", "a, b, c"].iter().enumerate() {
        assert_eq!(
            answer[i], actual[i],
            "wrong rule for the call `rule!({call})`"
        );
    }

    let calls = Cell::new(0);
    let next = || {
        calls.set(calls.get() + 1);
        calls.get()
    };
    let by_macro = twice!(next());
    let macro_calls = calls.get();
    calls.set(0);
    assert_eq!(
        twice_by_hand(&next),
        by_macro,
        "the value differs from the macro's"
    );
    assert_eq!(
        calls.get(),
        macro_calls,
        "the macro calls `next` more than once"
    );
    println!("all checks passed");
}
