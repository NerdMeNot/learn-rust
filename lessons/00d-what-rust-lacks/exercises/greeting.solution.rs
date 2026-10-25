// Other languages write greet(name, greeting = "Hello"). Rust has no default arguments,
// so the caller passes an Option and the function supplies the default.
// An empty greeting is still a greeting: only None means "use the default".
fn greet(name: &str, greeting: Option<&str>) -> String {
    let greeting = greeting.unwrap_or("Hello");
    format!("{greeting}, {name}!")
}

fn main() {
    assert_eq!(greet("Ada", None), "Hello, Ada!");
    assert_eq!(greet("Ada", Some("Hi")), "Hi, Ada!");
    assert_eq!(greet("Ada", Some("")), ", Ada!");
    assert_eq!(greet("", None), "Hello, !");
    println!("all checks passed");
}
