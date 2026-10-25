fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |x| x + n
}

fn main() {
    println!("{}", make_adder(5)(1));
}
