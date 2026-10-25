use std::rc::Rc;

struct Config {
    name: String,
}

struct Worker {
    config: Rc<Config>, // keeps its own share
}

// reads only: a plain reference is enough, and works for any owner
fn describe(config: &Config) -> String {
    format!("config {}", config.name)
}

// keeps a share: takes an Rc by value, so the caller decides to clone
fn hire(config: Rc<Config>) -> Worker {
    Worker { config }
}

fn main() {
    let config = Rc::new(Config {
        name: String::from("prod"),
    });
    println!("{}", describe(&config)); // &Rc<Config> becomes &Config
    println!("owners: {}", Rc::strong_count(&config));

    let a = hire(Rc::clone(&config));
    let b = hire(Rc::clone(&config));
    println!("owners: {}", Rc::strong_count(&config));
    println!("{} and {}", describe(&a.config), describe(&b.config));
}
