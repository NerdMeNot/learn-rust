fn total(xs: &[i32]) -> i32 {
    let mut sum = 0;
    for x in xs {
        sum += x;
    }
    sum
}

fn largest(xs: &[i32]) -> i32 {
    let mut best = xs[0];
    for x in xs {
        if *x > best {
            best = *x;
        }
    }
    best
}

fn main() {
    let scores = vec![30, 95, 60];

    // Three borrows, all alive at once.
    let a = &scores;
    let b = &scores;
    let c = &scores[1..];
    println!("a and b are the same Vec: {}", std::ptr::eq(a, b));

    println!("total(a): {}", total(a));
    println!("largest(b): {}", largest(b));
    println!("total(c): {}", total(c));
    println!("scores: {scores:?}");
}
