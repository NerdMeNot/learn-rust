struct Animal {
    name: String,
}

impl Animal {
    fn describe(&self) -> String {
        format!("{} is an animal", self.name)
    }
}

struct Dog {
    animal: Animal,
    #[allow(dead_code)]
    tricks: u32,
}

impl Dog {
    fn describe(&self) -> String {
        self.animal.describe() // forward by hand
    }
}

fn main() {
    let d = Dog {
        animal: Animal { name: "rex".into() },
        tricks: 2,
    };
    println!("{} | {}", d.animal.name, d.describe());
}
