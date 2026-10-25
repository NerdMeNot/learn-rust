#[derive(Debug)]
struct Config {
    name: String,
    retries: u32,
    verbose: bool,
}

fn main() {
    let base = Config {
        name: String::from("base"),
        retries: 3,
        verbose: false,
    };

    // `name` is not given, so `..base` moves it out of `base`.
    let loud = Config {
        verbose: true,
        ..base
    };
    println!("loud: {loud:?}");
    println!("base: {base:?}");
}
