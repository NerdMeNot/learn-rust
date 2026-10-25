use std::mem::size_of;

fn log(msg: &str) {
    println!("log: {msg}");
}

fn main() {
    let mut x = 1;
    println!("x starts as {x}");
    let from_fn = log("hello");
    let from_macro = println!("printing");
    let from_assign = x = 5;
    let from_add = x += 1;
    let from_while = while x < 10 {
        x += 1;
    };
    let from_for = for _ in 0..2 {};

    println!("fn: {from_fn:?}");
    println!("println!: {from_macro:?}");
    println!("assignment: {from_assign:?}");
    println!("+=: {from_add:?}");
    println!("while: {from_while:?}");
    println!("for: {from_for:?}");
    println!("size_of::<()>(): {}", size_of::<()>());
}
