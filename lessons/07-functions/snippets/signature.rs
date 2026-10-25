fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn greet(name: &str) {
    println!("hello, {name}");
}

fn main() {
    println!("add(2, 3): {}", add(2, 3));
    greet("Ferris");
}
