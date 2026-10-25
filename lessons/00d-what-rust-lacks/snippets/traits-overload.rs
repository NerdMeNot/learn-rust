trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("int {self}")
    }
}

impl Show for &str {
    fn show(&self) -> String {
        format!("text {self}")
    }
}

fn main() {
    println!("{}", 1.show());
    println!("{}", "a".show());
}
