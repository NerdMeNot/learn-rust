fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let extra = 100;
    println!("{}", apply(|n| n + extra, 4));
}
