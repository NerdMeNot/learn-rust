fn apply(f: impl Fn(i64) -> i64, x: i64) -> i64 {
    f(x)
}

fn main() {
    let mut count = 0;
    let result = apply(
        |x| {
            count += 1;
            x + count
        },
        1,
    );
    println!("result: {result}");
}
