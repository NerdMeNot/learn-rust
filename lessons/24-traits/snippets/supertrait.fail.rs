trait Speaker {
    fn speak(&self) -> String;
}

trait Pet: Speaker {
    fn name(&self) -> String;
}

struct Cat;

impl Pet for Cat {
    fn name(&self) -> String {
        String::from("Tom")
    }
}

fn main() {
    println!("{}", Cat.name());
}
