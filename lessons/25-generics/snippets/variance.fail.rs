trait Animal {}

struct Dog;

impl Animal for Dog {}

fn main() {
    let _one: Box<dyn Animal> = Box::new(Dog); // a Dog is an Animal
    let dogs: Vec<Box<Dog>> = vec![Box::new(Dog)];
    let animals: Vec<Box<dyn Animal>> = dogs; // a list of Dogs is not a list of Animals
    println!("{}", animals.len());
}
