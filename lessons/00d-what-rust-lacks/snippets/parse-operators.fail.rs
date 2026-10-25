fn increment() {
    let mut count = 0;
    count++;
}

fn prefix() {
    let mut count = 0;
    ++count;
}

fn decrement() {
    let mut count = 10;
    count--;
}

fn ternary(n: i32) -> &'static str {
    n % 2 == 0 ? "even" : "odd"
}

fn main() {}
