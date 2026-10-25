trait Speaker {
    fn speak(&self) -> String;
}

struct Dog;

impl Dog {
    fn speak(&self) -> String {
        "woof".to_string()
    }
}

fn talk<T: Speaker>(t: &T) -> String {
    t.speak()
}

fn main() {
    println!("{}", talk(&Dog));
}
