use std::fmt;

/// Prints as `#<number>`, for example `#42`.
struct Tag(u32);

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("#{}", self.0))
    }
}

fn main() {
    assert_eq!(Tag(42).to_string(), "#42");
    assert_eq!(format!("[{:>8}]", Tag(42)), "[     #42]");
    assert_eq!(format!("[{:<6}]", Tag(7)), "[#7    ]");
    assert_eq!(format!("[{:*^7}]", Tag(7)), "[**#7***]");
    assert_eq!(format!("[{:3}]", Tag(12345)), "[#12345]"); // wider than the width: not cut
    assert_eq!(format!("{:<6}|{:>6}|", Tag(1), Tag(22)), "#1    |   #22|");
    println!("all checks passed");
}
