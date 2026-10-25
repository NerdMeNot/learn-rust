use std::cell::Cell;

macro_rules! square_twice {
    ($x:expr) => {
        $x * $x
    };
}
macro_rules! square_once {
    ($x:expr) => {{
        let v = $x;
        v * v
    }};
}
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let calls = Cell::new(0);
    let next = || {
        calls.set(calls.get() + 1);
        calls.get()
    };

    let a = square_twice!(next());
    println!("square_twice!(next()) = {a} after {} calls", calls.get());
    calls.set(0);
    let b = square_once!(next());
    println!("square_once!(next())  = {b} after {} calls", calls.get());
    calls.set(0);
    let c = square(next());
    println!("square(next())        = {c} after {} calls", calls.get());
}
