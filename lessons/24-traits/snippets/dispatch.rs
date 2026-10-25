use std::any::type_name;

trait Speaker {
    fn speak(&self) -> String;
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

// Generic: T is one concrete type, known when the program is compiled.
fn generic<T: Speaker>(s: &T) {
    println!("generic: T = {}, says {}", type_name::<T>(), s.speak());
}

fn main() {
    generic(&Dog);
    generic(&Robot { id: 7 });

    // dyn: one list holding different types; the method is looked up while running.
    let crowd: Vec<Box<dyn Speaker>> = vec![Box::new(Dog), Box::new(Robot { id: 7 })];
    for s in &crowd {
        println!("dyn: {}", s.speak());
    }
}
