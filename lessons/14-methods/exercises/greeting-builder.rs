struct GreetingBuilder {
    name: String,
    excited: bool,
    times: u32,
}

impl GreetingBuilder {
    /// No name, not excited, said once.
    fn new() -> Self {
        todo!("name is String::new(), excited is false, times is 1")
    }

    /// Sets the name. Each setter takes `mut self` and gives `self` back.
    fn name(mut self, name: &str) -> Self {
        todo!("self.name = name.to_string(); self")
    }

    /// `true` ends the greeting with "!", `false` with ".".
    fn excited(mut self, yes: bool) -> Self {
        todo!("set the field, return self")
    }

    /// How many times the greeting is repeated.
    fn times(mut self, n: u32) -> Self {
        todo!("set the field, return self")
    }

    /// "Hello, NAME." repeated `times` times, separated by one space.
    /// The name is trimmed; a blank name becomes "stranger".
    /// Zero times gives an empty String.
    fn build(self) -> String {
        todo!("trim the name, make one sentence, push_str it `times` times with spaces between")
    }
}

fn main() {
    assert_eq!(GreetingBuilder::new().build(), "Hello, stranger.");
    assert_eq!(
        GreetingBuilder::new().name("Zoë").excited(true).build(),
        "Hello, Zoë!"
    );
    assert_eq!(
        GreetingBuilder::new().times(2).name("  Åsa  ").build(),
        "Hello, Åsa. Hello, Åsa."
    );
    assert_eq!(
        GreetingBuilder::new().name("   ").build(),
        "Hello, stranger.",
        "a blank name falls back to stranger"
    );
    assert_eq!(GreetingBuilder::new().times(0).build(), "");

    // Setting an option only sometimes means putting the builder back.
    let loud = true;
    let mut builder = GreetingBuilder::new().name("Li");
    if loud {
        builder = builder.excited(true);
    }
    assert_eq!(builder.build(), "Hello, Li!");

    println!("all checks passed");
}
