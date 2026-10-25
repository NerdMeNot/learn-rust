use std::mem::size_of;

trait Speaker {
    fn speak(&self) -> String;
}

struct Dog;
struct Robot {
    id: u32,
}

impl Speaker for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

impl Speaker for Robot {
    fn speak(&self) -> String {
        format!("beep {}", self.id)
    }
}

fn main() {
    println!("size_of::<Dog>(): {}", size_of::<Dog>());
    println!("size_of::<Robot>(): {}", size_of::<Robot>());
    println!("size_of::<u32>(): {}", size_of::<u32>());
    println!("{} {}", Dog.speak(), Robot { id: 1 }.speak());
}
