fn apply_ptr(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let offset = 10;
    println!("{}", apply_ptr(|x| x + offset, 5));
}
