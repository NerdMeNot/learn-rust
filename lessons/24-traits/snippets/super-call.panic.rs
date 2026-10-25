trait Speaker {
    fn speak(&self) -> String;

    fn shout(&self) -> String {
        self.speak().to_uppercase()
    }
}

struct Parrot;

impl Speaker for Parrot {
    fn speak(&self) -> String {
        String::from("hello")
    }

    // There is no `super` call. This names Parrot's own shout again.
    #[allow(unconditional_recursion)]
    fn shout(&self) -> String {
        format!("{}!!", Speaker::shout(self))
    }
}

fn main() {
    println!("{}", Parrot.shout());
}
