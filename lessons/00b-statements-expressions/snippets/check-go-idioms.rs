// The Rust sides of the Go habits in the table: go-idioms.go prints the same facts.
use std::mem::size_of;

fn f() -> i32 {
    5
}

fn named() -> i32 {
    let n = 3;
    n
}

fn main() {
    let mut i = 0;
    i += 1;
    println!("i++: {i}");
    let v = f();
    if v > 3 {
        println!("init statement: {v}");
    }
    println!("struct{{}} size: {}", size_of::<()>());
    println!("named result: {}", named());
}
