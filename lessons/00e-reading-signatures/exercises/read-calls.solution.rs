// Read the signatures, then say what each call gives. Replace each `todo!()` and run
// until the checks print `all checks passed`.
//
// Two methods you have not met: `list.first()` is the first item of a slice, as an
// `Option<&i32>` (`None` when the slice is empty), and `.copied()` turns an
// `Option<&i32>` into an `Option<i32>` by copying the number out of the borrow.
fn bump(x: &mut i32) {
    *x += 1;
}

fn twice(mut f: impl FnMut() -> i32) -> i32 {
    f() + f()
}

fn pick<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn head(list: &[i32]) -> Option<&i32> {
    list.first()
}

fn calls() -> (i32, i32, &'static str, Option<i32>, Option<i32>) {
    let mut n = 5;
    bump(&mut n);
    bump(&mut n);
    let mut c = 0;
    let t = twice(|| {
        c += 1;
        c
    });
    let (full, empty) = ([8, 9], []);
    (
        n,
        t,
        pick("pear", "zebra"),
        head(&full).copied(),
        head(&empty).copied(),
    )
}

/// Hint: `bump` takes `&mut i32`: does the caller's `n` change?
fn n_after_bumps() -> i32 {
    7
}

/// Hint: `impl FnMut` may keep state between calls: what does the closure return each time?
fn twice_gives() -> i32 {
    3
}

/// Hint: `T: PartialOrd` lets `pick` use `>`: which of the two words is larger?
fn pick_gives() -> &'static str {
    "zebra"
}

/// Hint: the result is an `Option` of a borrow: which item is first?
fn head_of_two_gives() -> Option<i32> {
    Some(8)
}

/// Hint: the list is empty
fn head_of_none_gives() -> Option<i32> {
    None
}

fn check<T: PartialEq + std::fmt::Debug>(call: &str, said: T, truth: T) {
    assert!(said == truth, "misread `{call}`: you said {said:?}");
    println!("ok: {call} = {said:?}");
}

fn main() {
    let (n, t, p, first, none) = calls();
    check("n after the two bumps", n_after_bumps(), n);
    check("twice(..) for the counting closure", twice_gives(), t);
    check("pick(\"pear\", \"zebra\")", pick_gives(), p);
    check("head of [8, 9]", head_of_two_gives(), first);
    check("head of an empty list", head_of_none_gives(), none);
    println!("all checks passed");
}
