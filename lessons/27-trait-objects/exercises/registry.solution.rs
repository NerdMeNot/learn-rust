use std::collections::HashMap;

trait Command {
    fn run(&mut self, arg: &str) -> String;
}

struct Shout;

struct Counter {
    calls: u32,
}

struct Repeat {
    times: usize,
}

impl Command for Shout {
    fn run(&mut self, arg: &str) -> String {
        arg.to_uppercase()
    }
}

impl Command for Counter {
    fn run(&mut self, _arg: &str) -> String {
        self.calls += 1;
        format!("call {}", self.calls)
    }
}

impl Command for Repeat {
    fn run(&mut self, arg: &str) -> String {
        arg.repeat(self.times)
    }
}

struct Registry {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Registry {
    fn new() -> Registry {
        Registry {
            commands: HashMap::new(),
        }
    }

    fn register(&mut self, name: &str, command: Box<dyn Command>) {
        self.commands.insert(name.to_string(), command);
    }

    fn run(&mut self, name: &str, arg: &str) -> Option<String> {
        let command = self.commands.get_mut(name)?;
        Some(command.run(arg))
    }
}

fn main() {
    let mut reg = Registry::new();
    reg.register("shout", Box::new(Shout));
    reg.register("count", Box::new(Counter { calls: 0 }));
    reg.register("twice", Box::new(Repeat { times: 2 }));

    assert_eq!(reg.run("shout", "hi"), Some("HI".to_string()));
    assert_eq!(reg.run("twice", "ab"), Some("abab".to_string()));

    // a command keeps its own state between calls
    assert_eq!(reg.run("count", ""), Some("call 1".to_string()));
    assert_eq!(reg.run("count", ""), Some("call 2".to_string()));

    // an unknown name is None, not a panic
    assert_eq!(reg.run("nope", "x"), None);

    // registering a name again replaces the old command, state and all
    reg.register("count", Box::new(Counter { calls: 10 }));
    assert_eq!(reg.run("count", ""), Some("call 11".to_string()));

    println!("all checks passed");
}
