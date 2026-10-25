mod zoo {
    pub trait Speaker {
        fn speak(&self) -> String;
    }

    pub struct Dog;

    impl Speaker for Dog {
        fn speak(&self) -> String {
            String::from("woof")
        }
    }
}

// No `use` here: the bound names the trait, so t.speak() is reachable.
fn generic<T: zoo::Speaker>(t: &T) -> String {
    t.speak()
}

// The same for a trait object.
fn dynamic(t: &dyn zoo::Speaker) -> String {
    t.speak()
}

mod kennel {
    // `as _` brings the trait into scope for method calls without taking its name.
    use crate::zoo::Speaker as _;

    pub fn call() -> String {
        crate::zoo::Dog.speak()
    }
}

fn main() {
    println!("generic: {}", generic(&zoo::Dog));
    println!("dyn: {}", dynamic(&zoo::Dog));
    println!("kennel: {}", kennel::call());
}
