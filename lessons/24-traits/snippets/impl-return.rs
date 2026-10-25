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

// One concrete type per function: the caller sees only "a Speaker".
fn make_dog() -> impl Speaker {
    Dog
}

// Different types chosen at run time need a trait object.
fn pick(dog: bool) -> Box<dyn Speaker> {
    if dog { Box::new(Dog) } else { Box::new(Robot) }
}

fn main() {
    println!("make_dog(): {}", make_dog().speak());
    println!("pick(false): {}", pick(false).speak());
}
