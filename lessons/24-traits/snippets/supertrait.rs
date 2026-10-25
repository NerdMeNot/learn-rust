trait Speaker {
    fn speak(&self) -> String;
}

// A Pet must also be a Speaker: Pet's own methods may call speak().
trait Pet: Speaker {
    fn name(&self) -> String;

    fn introduce(&self) -> String {
        format!("{} says {}", self.name(), self.speak())
    }
}

struct Dog;

impl Speaker for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        String::from("Rex")
    }
}

// A bound on Pet also gives access to Speaker's methods.
fn loud<P: Pet>(p: &P) -> String {
    p.speak().to_uppercase()
}

fn main() {
    println!("introduce(): {}", Dog.introduce());
    println!("loud(): {}", loud(&Dog));
}
