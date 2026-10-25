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
}

fn main() {
    let d = Dog {
        animal: Animal { name: "rex".into() },
    };
    println!("{} | {}", d.name, d.describe());
}
