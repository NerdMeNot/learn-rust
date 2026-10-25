trait Speaker {
    fn speak(&self) -> String;

    fn shout(&self) -> String {
        // default method
        self.speak().to_uppercase()
    }
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

fn introduce(s: &dyn Speaker) {
    // any type that implements Speaker
    println!("{} / {}", s.speak(), s.shout());
}

fn main() {
    introduce(&Dog);
    introduce(&Robot { id: 7 });
}
