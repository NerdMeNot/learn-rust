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

fn main() {
    let dog = zoo::Dog;
    println!("{}", dog.speak());
}
