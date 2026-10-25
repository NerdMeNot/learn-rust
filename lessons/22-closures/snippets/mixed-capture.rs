fn main() {
    let x = 2;
    let mut y = 1;
    // `move` takes every variable the body uses, so make the reference first:
    // the closure then owns `x` (a copy) and the reference to `y`.
    let y_ref = &mut y;
    let mut add = move || {
        *y_ref += x;
        *y_ref
    };
    println!("add(): {}", add());
    println!("add(): {}", add());
    println!("y after the closure's last use: {y}");
}
