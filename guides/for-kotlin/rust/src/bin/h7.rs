trait Speaker {
    fn speak(&self) -> String;
}

impl Speaker for i32 {
    fn speak(&self) -> String {
        format!("number {self}")
    }
}

fn announce(s: &impl Speaker) {
    println!("{}", s.speak());
}

fn main() {
    let n: i32 = 21;
    println!("{}", n.speak());
    announce(&n);
}
