trait Animal {
    fn name(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> String {
        "dog".to_string()
    }
}

impl Animal for Cat {
    fn name(&self) -> String {
        "cat".to_string()
    }
}

// Java's `List<? extends Animal>`: any one type that is an Animal.
fn names<T: Animal>(xs: &[T]) -> Vec<String> {
    xs.iter().map(|x| x.name()).collect()
}

// A list that mixes kinds holds trait objects.
fn mixed_names(xs: &[Box<dyn Animal>]) -> Vec<String> {
    xs.iter().map(|x| x.name()).collect()
}

fn main() {
    println!("dogs: {:?}", names(&[Dog, Dog]));
    let pets: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    println!("mixed: {:?}", mixed_names(&pets));
}
