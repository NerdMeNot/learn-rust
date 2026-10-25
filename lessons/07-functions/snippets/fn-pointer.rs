fn double(n: i32) -> i32 {
    n * 2
}

fn negate(n: i32) -> i32 {
    -n
}

fn apply(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    println!("apply(double, 4): {}", apply(double, 4));
    println!("apply(negate, 4): {}", apply(negate, 4));
    // A closure that captures nothing converts to a fn pointer.
    println!("apply(|n| n + 100, 4): {}", apply(|n| n + 100, 4));

    let ops: [fn(i32) -> i32; 2] = [double, negate];
    for f in ops {
        println!("f(10): {}", f(10));
    }
}
