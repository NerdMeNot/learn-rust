#[derive(Debug, Display)]
struct Version {
    major: u32,
    minor: u32,
}

fn main() {
    let v = Version { major: 1, minor: 2 };
    println!("{v:?}");
}
