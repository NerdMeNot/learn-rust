use std::fmt;

#[derive(Debug)]
struct Version {
    major: u32,
    minor: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}

struct Tag(String);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&self.0)
    }
}

fn main() {
    let v = Version { major: 1, minor: 2 };
    println!("Display: {v}");
    println!("Debug: {v:?}");
    let text = v.to_string();
    println!("to_string: {text}");
    println!("width 8 with write!: [{v:>8}]");
    println!("width 8 with pad:    [{:>8}]", Tag(String::from("v1.2")));
}
