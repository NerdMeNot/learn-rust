fn call_twice(f: impl Fn()) {
    f();
    f();
}

fn main() {
    let mut count = 0;
    call_twice(|| count += 1);
    println!("count: {count}");
}
