trait Speaker {
    fn speak(&self) -> String;
}

struct Robot;

impl Speaker for Robot {
    fn speak(&self) -> String {
        "beep".to_string()
    }
}

fn introduce(s: &impl Speaker) {
    println!("{}", s.speak());
}

fn main() {
    introduce(&Robot);
}
