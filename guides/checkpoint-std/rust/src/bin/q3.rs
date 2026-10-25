use std::fmt;

struct Tag(u32);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

struct Name(&'static str);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(self.0)
    }
}

fn main() {
    println!("[{:>6}]", Tag(42));
    println!("[{:>6}]", Name("ada"));
    println!("[{:>6}]", format!("{}", Tag(42)));
    println!("[{:.2}]", Name("ada"));
}
