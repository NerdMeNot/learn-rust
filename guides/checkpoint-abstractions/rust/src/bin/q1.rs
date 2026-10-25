trait Pilot {
    fn fly(&self) -> String;
}

trait Wizard {
    fn fly(&self) -> String;
}

struct Human;

impl Pilot for Human {
    fn fly(&self) -> String {
        String::from("pilot: this is your captain")
    }
}

impl Wizard for Human {
    fn fly(&self) -> String {
        String::from("wizard: up!")
    }
}

impl Human {
    fn fly(&self) -> String {
        String::from("human: *waves arms*")
    }
}

fn main() {
    let person = Human;
    println!("{}", person.fly());
    println!("{}", Pilot::fly(&person));
    println!("{}", <Human as Wizard>::fly(&person));
}
