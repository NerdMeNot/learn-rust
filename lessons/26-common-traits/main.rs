use std::collections::HashSet;
use std::fmt;

// Each name in `derive` asks the compiler to write one standard ability for us.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct Version {
    major: u32,
    minor: u32,
}

// Display (the `{}` text for users) cannot be derived: we choose how it looks.
impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}", self.major, self.minor)
    }
}

fn main() {
    let a = Version {
        major: 1,
        minor: 10,
    };
    let b = Version { major: 1, minor: 9 };
    println!("display: {a}");
    println!("a == b: {}", a == b);
    println!("a > b: {}", a > b);

    let mut all = vec![a, b, Version::default()];
    all.sort();
    let names: Vec<String> = all.iter().map(|v| v.to_string()).collect();
    println!("sorted: {}", names.join(" "));

    let mut seen = HashSet::new();
    for v in [a, b, Version::default(), a] {
        seen.insert(v);
    }
    println!("distinct: {}", seen.len());
}
