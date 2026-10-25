fn largest(xs: &Vec<i32>) -> i32 {
    let mut best = xs[0];
    for x in xs {
        if *x > best {
            best = *x;
        }
    }
    best
}

fn main() {
    let array = [7, 1, 8];
    println!("largest(&array): {}", largest(&array));
}
