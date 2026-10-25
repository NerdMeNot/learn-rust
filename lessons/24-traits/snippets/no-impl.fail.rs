trait Speaker {
    fn speak(&self) -> String;
}

struct Dog;
struct Rock;

impl Speaker for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

fn introduce<T: Speaker>(s: &T) {
    println!("{}", s.speak());
}

fn main() {
    introduce(&Dog);
    introduce(&Rock);
}
