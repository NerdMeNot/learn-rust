use std::fmt::Display;

fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn show(x: impl Display) -> String {
    format!("<{x}>")
}

fn main() {
    let extra = 100;
    println!("apply(|n| n + extra, 4): {}", apply(|n| n + extra, 4));
    println!("show(5): {}", show(5));
    println!("show(\"hi\"): {}", show("hi"));
}
