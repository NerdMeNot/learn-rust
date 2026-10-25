static NAMES: [&str; 2] = ["ana", "bo"];

fn first_name() -> &'static str {
    NAMES[0]
}

fn leaked() -> &'static str {
    // gives the String away for the rest of the run; it is never freed
    Box::leak(String::from("never freed").into_boxed_str())
}

fn main() {
    println!("first_name: {}", first_name());
    println!("leaked: {}", leaked());
}
