// Greet someone, then add up 1 to 4.
fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", greet("Ferris"));
    let mut total = 0;
    for n in 1..=4 {
        total = add(total, n);
    }
    println!("total: {total}");
}
