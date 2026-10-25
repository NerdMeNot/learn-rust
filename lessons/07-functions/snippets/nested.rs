fn describe(n: i32) -> String {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    let kind = if is_even(n) { "even" } else { "odd" };
    format!("{n} is {kind}")
}

fn main() {
    println!("{}", describe(7));
    println!("{}", describe(10));
}
