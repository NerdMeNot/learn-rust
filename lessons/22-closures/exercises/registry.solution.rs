// A registry of named callbacks. `on` stores a handler under a name; `fire` calls every
// handler stored under that name, in the order they were added, and collects what they
// return. A name with no handlers gives an empty Vec.
struct Registry {
    handlers: Vec<(String, Box<dyn Fn(&str) -> String>)>,
}

impl Registry {
    fn new() -> Registry {
        Registry {
            handlers: Vec::new(),
        }
    }

    // `'static` means the closure may not borrow local variables: it must own what it uses.
    fn on(&mut self, name: &str, handler: impl Fn(&str) -> String + 'static) {
        self.handlers.push((name.to_string(), Box::new(handler)));
    }

    fn fire(&self, name: &str, arg: &str) -> Vec<String> {
        let mut results = Vec::new();
        for (registered, handler) in &self.handlers {
            if registered == name {
                results.push(handler(arg));
            }
        }
        results
    }
}

fn main() {
    let mut registry = Registry::new();
    let prefix = String::from("hello");
    registry.on("greet", move |who| format!("{prefix}, {who}"));
    registry.on("greet", |who| who.to_uppercase());
    registry.on("count", |text| text.len().to_string());
    registry.on("greeting", |_| String::from("not a greet"));
    registry.on("", |_| String::from("empty name"));

    assert_eq!(registry.fire("greet", "ana"), ["hello, ana", "ANA"]);
    assert_eq!(registry.fire("count", "hello"), ["5"]);
    assert!(registry.fire("missing", "x").is_empty());
    // Only an exact name matches: "greet" is a prefix of "greeting", "count" is not.
    assert_eq!(registry.fire("greeting", "x"), ["not a greet"]);
    assert_eq!(registry.fire("gree", "x"), Vec::<String>::new());
    assert_eq!(registry.fire("", "x"), ["empty name"]);

    let empty = Registry::new();
    assert!(empty.fire("greet", "x").is_empty());
    println!("all checks passed");
}
