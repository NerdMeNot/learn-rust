fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply_generic<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn apply_dyn(f: &dyn Fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn apply_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn repeat(n: u32, mut f: impl FnMut()) {
    for _ in 0..n {
        f();
    }
}

fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    let offset = 10;
    println!("apply: {}", apply(|x| x + offset, 5));
    println!("apply_generic: {}", apply_generic(|x| x + offset, 5));
    println!("apply_dyn: {}", apply_dyn(&|x| x + offset, 5));
    println!("apply with a named fn: {}", apply(double, 5));
    println!("apply_ptr, named fn: {}", apply_ptr(double, 5));
    println!(
        "apply_ptr, closure with no captures: {}",
        apply_ptr(|x| x - 1, 5)
    );

    let mut calls = 0;
    repeat(3, || calls += 1);
    println!("repeat(3) made {calls} calls");
    repeat(0, || calls += 1);
    println!("repeat(0) made {calls} calls in total");
}
