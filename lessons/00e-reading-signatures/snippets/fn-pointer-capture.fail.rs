fn apply(f: fn(i64) -> i64, x: i64) -> i64 {
    f(x)
}

fn main() {
    let offset = 10;
    println!("result: {}", apply(|x| x + offset, 1));
}
