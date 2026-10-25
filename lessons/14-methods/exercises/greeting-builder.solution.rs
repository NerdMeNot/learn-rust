struct GreetingBuilder {
    name: String,
    excited: bool,
    times: u32,
}

impl GreetingBuilder {
    /// No name, not excited, said once.
    fn new() -> Self {
        Self {
            name: String::new(),
            excited: false,
            times: 1,
        }
    }

    /// Sets the name. Each setter takes `mut self` and gives `self` back.
    fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// `true` ends the greeting with "!", `false` with ".".
    fn excited(mut self, yes: bool) -> Self {
        self.excited = yes;
        self
    }

    /// How many times the greeting is repeated.
    fn times(mut self, n: u32) -> Self {
        self.times = n;
        self
    }

    /// "Hello, NAME." repeated `times` times, separated by one space.
    /// The name is trimmed; a blank name becomes "stranger".
    /// Zero times gives an empty String.
    fn build(self) -> String {
        let name = self.name.trim();
        let name = if name.is_empty() { "stranger" } else { name };
        let end = if self.excited { "!" } else { "." };
        let sentence = format!("Hello, {name}{end}");
        let mut out = String::new();
        for i in 0..self.times {
            if i > 0 {
                out.push(' ');
            }
            out.push_str(&sentence);
        }
        out
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
