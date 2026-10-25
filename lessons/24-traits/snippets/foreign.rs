trait Describe {
    fn describe(&self) -> String;
}

impl Describe for i32 {
    fn describe(&self) -> String {
        format!("the number {}", self)
    }
}

impl Describe for &str {
    fn describe(&self) -> String {
        format!("the text {:?}", self)
    }
}

impl<T: Describe> Describe for Vec<T> {
    fn describe(&self) -> String {
        let parts: Vec<String> = self.iter().map(|item| item.describe()).collect();
        format!("a list of [{}]", parts.join(", "))
    }
}

fn main() {
    println!("{}", 7.describe());
    println!("{}", "hi".describe());
    println!("{}", vec![1, 2].describe());
}
