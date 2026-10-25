trait Speaker {
    fn speak(&self) -> String;
    fn volume(&self) -> u8;
}

struct Dog;

impl Speaker for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

fn main() {
    println!("{}", Dog.speak());
}
