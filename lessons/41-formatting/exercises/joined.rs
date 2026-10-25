use std::fmt::{self, Display};

/// Prints the items with `Display`, separated by the second field. Whatever the caller
/// put after the colon (`{:.1}`, `{:>4}`) applies to every item.
struct Joined<'a, T>(&'a [T], &'a str);

impl<T: Display> Display for Joined<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        todo!(
            "write each item, with the separator between them, so that the caller's spec reaches the items"
        )
    }
}

/// An id that pads itself (it uses `f.pad`, so a width and a fill reach it).
struct Id(u32);

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!("#{}", self.0))
    }
}

fn main() {
    assert_eq!(Joined(&[1, 2, 3], ", ").to_string(), "1, 2, 3");
    assert_eq!(Joined(&["a", "b"], " | ").to_string(), "a | b"); // Display: no quotes
    assert_eq!(Joined(&[7], ", ").to_string(), "7");
    assert_eq!(Joined::<i32>(&[], ", ").to_string(), "");
    assert_eq!(format!("{:.1}", Joined(&[1.0, 2.26], ", ")), "1.0, 2.3");
    assert_eq!(
        format!("{:>4}", Joined(&[1, 22, 333], "|")),
        "   1|  22| 333"
    );
    assert_eq!(format!("{:>3}", Joined(&["a", "b"], ", ")), "  a,   b");
    assert_eq!(
        format!("{:>5}", Joined(&[Id(1), Id(22)], ";")),
        "   #1;  #22"
    );
    println!("all checks passed");
}
