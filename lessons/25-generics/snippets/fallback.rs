trait Describe {
    // The default method is the fallback.
    fn describe(&self) -> String {
        "something".to_string()
    }
}

impl Describe for i32 {
    fn describe(&self) -> String {
        "an integer".to_string()
    }
}

impl Describe for char {} // takes the fallback

fn main() {
    println!("5: {}", 5_i32.describe());
    println!("'x': {}", 'x'.describe());
}
