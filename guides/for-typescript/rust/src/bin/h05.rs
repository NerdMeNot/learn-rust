trait Speaker {
    fn speak(&self) -> String;
}

struct Robot;

impl Robot {
    fn speak(&self) -> String {
        "beep".to_string()
    }
}

fn say(s: &impl Speaker) {
    println!("{}", s.speak());
}

fn main() {
    say(&Robot); // Robot has a speak method, but never said impl Speaker
}
