trait Speaker {
    fn speak(&self) -> String; // required: every type writes its own

    fn shout(&self) -> String {
        // default: built on the required method
        self.speak().to_uppercase()
    }
}

struct Dog;
struct Robot {
    id: u32,
}
struct Parrot {
    phrase: String,
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

impl Speaker for Parrot {
    fn speak(&self) -> String {
        self.phrase.clone()
    }

    fn shout(&self) -> String {
        // replaces the default
        format!("{}!!", self.speak())
    }
}

fn main() {
    let robot = Robot { id: 7 };
    let parrot = Parrot {
        phrase: String::from("hello"),
    };
    println!("dog.speak(): {}", Dog.speak());
    println!("dog.shout(): {}", Dog.shout());
    println!("robot.shout(): {}", robot.shout());
    println!("parrot.shout(): {}", parrot.shout());
}
