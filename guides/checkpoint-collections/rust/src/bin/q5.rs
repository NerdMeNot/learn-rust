fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    |x| x + n
}

fn main() {
    let add5 = make_adder(5);
    println!("{}", add5(1));
}
