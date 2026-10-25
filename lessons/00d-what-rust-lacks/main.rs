// What other languages write with ?:, i++, null and switch, written in Rust.
fn size(n: i32) -> &'static str {
    match n {
        1 | 2 => "small",
        3 => "medium",
        _ => "large",
    }
}

fn first_even(items: &[i32]) -> Option<i32> {
    for &x in items {
        if x % 2 == 0 {
            return Some(x);
        }
    }
    None
}

fn main() {
    let mut total = 0;
    for i in 0..4 {
        total += i;
    }
    let label = if total > 5 { "big" } else { "small" };
    println!("total: {total} ({label})");
    println!("size: {} {}", size(2), size(9));
    match first_even(&[3, 5, 8]) {
        Some(n) => println!("first even: {n}"),
        None => println!("no even number"),
    }
    println!("none: {}", first_even(&[1, 3]).unwrap_or(-1));
    let wide = total as i64 * 1_000_000_000;
    println!("wide: {wide}");
}
