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

fn main() {
    println!("{}", Dog.speak());
    println!("{}", Rock.speak());
}
