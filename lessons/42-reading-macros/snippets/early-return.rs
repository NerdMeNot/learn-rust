macro_rules! parse_or_zero {
    ($s:expr) => {
        match $s.parse::<i32>() {
            Ok(n) => n,
            Err(_) => return 0,
        }
    };
}

fn sum_pair(a: &str, b: &str) -> i32 {
    let x = parse_or_zero!(a);
    let y = parse_or_zero!(b);
    println!("  both parsed: {x} and {y}");
    x + y
}

fn main() {
    println!("sum_pair(\"1\", \"2\") = {}", sum_pair("1", "2"));
    println!("sum_pair(\"1\", \"x\") = {}", sum_pair("1", "x"));
}
