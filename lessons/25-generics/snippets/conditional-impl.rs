use std::fmt::Display;

struct Wrapper<T>(T);

// For every T.
impl<T> Wrapper<T> {
    fn into_inner(self) -> T {
        self.0
    }
}

// Only when T can be displayed.
impl<T: Display> Wrapper<T> {
    fn show(&self) -> String {
        format!("[{}]", self.0)
    }
}

fn main() {
    println!("number: {}", Wrapper(42).show());
    println!("text: {}", Wrapper("hi").show());
    let list = Wrapper(vec![1, 2]);
    println!("a list still unwraps: {:?}", list.into_inner());
}
