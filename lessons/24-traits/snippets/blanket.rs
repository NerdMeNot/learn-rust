use std::fmt::Display;

trait Shout {
    fn shout(&self) -> String;
}

// Every type that implements Display gets Shout, with no per-type impl.
impl<T: Display> Shout for T {
    fn shout(&self) -> String {
        self.to_string().to_uppercase() + "!"
    }
}

struct Robot {
    id: u32,
}

impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "robot {}", self.id)
    }
}

fn main() {
    println!("&str: {}", "hello".shout());
    println!("i32: {}", 42.shout());
    println!("Robot: {}", Robot { id: 7 }.shout());
}
