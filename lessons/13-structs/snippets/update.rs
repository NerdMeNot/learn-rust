#[derive(Clone)]
struct Config {
    name: String,
    retries: u32,
    verbose: bool,
}

fn show(label: &str, c: &Config) {
    println!(
        "{label}: name={}, retries={}, verbose={}",
        c.name, c.retries, c.verbose
    );
}

fn main() {
    let base = Config {
        name: String::from("base"),
        retries: 3,
        verbose: false,
    };

    // `name` is given, so `..base` only supplies `retries` and `verbose`.
    // Both are copied, so `base` stays whole.
    let loud = Config {
        name: String::from("loud"),
        verbose: true,
        ..base
    };
    show("loud", &loud);
    show("base", &base);

    // `..base.clone()` supplies `name` as a copy, so `base` is never touched.
    let renamed = Config {
        verbose: true,
        ..base.clone()
    };
    show("renamed", &renamed);
    show("base", &base);
}
