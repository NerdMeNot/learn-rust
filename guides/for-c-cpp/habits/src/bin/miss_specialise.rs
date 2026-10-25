trait Describe {
    fn describe(&self) -> String;
}

impl<T> Describe for T {
    fn describe(&self) -> String {
        String::from("something")
    }
}

impl Describe for i32 {
    fn describe(&self) -> String {
        String::from("an integer")
    }
}

fn main() {
    println!("{}", 5.describe());
}
