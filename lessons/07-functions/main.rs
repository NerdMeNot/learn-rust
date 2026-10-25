fn add(a: i32, b: i32) -> i32 {
    a + b // no semicolon: this is what the function returns
}

fn greet(name: &str) {       // no -> means it returns nothing
    println!("hi {name}");
}

fn main() {
    println!("{}", add(2, 3));
    greet("kotlin");
}
