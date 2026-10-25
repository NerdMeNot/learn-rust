use std::fmt;

// newtype!(Meters, f64) defines `struct Meters(f64)` with `new`, `get`, and a Display that
// prints `Meters(2.5)`: the type's own name, then the value in parentheses.
macro_rules! newtype {
    ($name:ident, $inner:ty) => {
        struct $name($inner);

        impl $name {
            fn new(value: $inner) -> Self {
                $name(value)
            }

            fn get(&self) -> &$inner {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

newtype!(Meters, f64);
newtype!(UserId, u32);
newtype!(Label, String);

fn main() {
    let m = Meters::new(2.5);
    assert_eq!(*m.get(), 2.5);
    assert_eq!(m.to_string(), "Meters(2.5)");

    let id = UserId::new(7);
    assert_eq!(*id.get(), 7);
    assert_eq!(format!("{id}"), "UserId(7)");

    let label = Label::new(String::from("hi"));
    assert_eq!(label.get(), "hi");
    assert_eq!(label.to_string(), "Label(hi)");
    println!("all checks passed");
}
