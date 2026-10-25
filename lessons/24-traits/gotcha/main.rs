trait Speaker {
    fn speak(&self) -> String;
}

struct Robot {
    id: u32,
}

impl Robot {
    fn speak(&self) -> String {
        format!("beep {}", self.id)
    }
}

fn introduce(s: &dyn Speaker) {
    println!("{}", s.speak());
}

fn main() {
    introduce(&Robot { id: 7 });
}
