struct Builder {
    name: String,
}

impl Builder {
    fn new() -> Self {
        Self {
            name: String::new(),
        }
    }

    fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    fn build(self) -> String {
        self.name
    }
}

fn main() {
    let text = Builder::new().name("ana").build();
    println!("built: {text}");
}
