trait Speaker {
    fn speak(&self) -> String;
}

struct Dog;
struct Robot;

impl Speaker for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

impl Speaker for Robot {
    fn speak(&self) -> String {
        String::from("beep")
    }
}

fn pick(dog: bool) -> impl Speaker {
    if dog { Dog } else { Robot }
}

fn main() {
    println!("{}", pick(true).speak());
}
