use std::sync::mpmc;

fn main() {
    let (_tx, _rx) = mpmc::channel::<i32>(); // many consumers: unstable on 1.99.1
}
