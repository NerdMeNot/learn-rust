use std::alloc::System;

fn main() {
    let mut v: Vec<i32, System> = Vec::new_in(System);
    v.push(1);
    println!("{v:?}");
}
