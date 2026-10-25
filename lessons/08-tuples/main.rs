fn min_max(xs: &[i32]) -> (i32, i32) {
    let mut lo = xs[0];
    let mut hi = xs[0];
    for &x in xs {
        // each item is &i32; the &x pattern takes the number out
        if x < lo {
            lo = x;
        }
        if x > hi {
            hi = x;
        }
    }
    (lo, hi)
}

fn main() {
    let (lo, hi) = min_max(&[4, 9, 1, 7]);
    println!("{lo} {hi}");

    let (_, only_hi) = min_max(&[3, 5]); // _ ignores a value
    println!("{only_hi}");
}
