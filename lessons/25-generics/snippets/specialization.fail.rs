trait Describe {
    fn describe(&self) -> String;
}

// The general case, like the primary template in C++.
impl<T> Describe for T {
    fn describe(&self) -> String {
        "something".to_string()
    }
}

// The special case, like `template <> struct Describe<int>`.
impl Describe for i32 {
    fn describe(&self) -> String {
        "an integer".to_string()
    }
}

fn main() {
    println!("{}", 5_i32.describe());
}
