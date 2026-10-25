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

fn main() {
    let crowd = vec![Dog, Robot];
    println!("{}", crowd.len());
}
