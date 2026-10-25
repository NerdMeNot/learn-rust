use std::fmt;

struct Rgb(u8, u8, u8);

impl fmt::LowerHex for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            f.write_str("#")?;
        }
        write!(f, "{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

fn main() {
    let orange = Rgb(255, 128, 0);
    println!("{:x}", orange);
    println!("{:#x}", orange);
    // Rgb has no Display, no Binary and no UpperHex, so {} or {:X} would not compile.
    println!("length of {{:x}}: {}", format!("{:x}", orange).len());
}
