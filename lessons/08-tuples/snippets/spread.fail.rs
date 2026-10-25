fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn pair() -> (i32, i32) {
    (3, 4)
}

fn main() {
    println!("{}", add(pair()));
}
