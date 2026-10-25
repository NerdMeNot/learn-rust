use std::fmt::{Debug, Display};

// Two bounds joined with `+`.
fn report<T: PartialOrd + Display>(items: &[T]) -> String {
    let mut best = &items[0];
    for item in items {
        if item > best {
            best = item;
        }
    }
    format!("largest of {}: {best}", items.len())
}

// Two `impl` parameters are two separate hidden type parameters.
fn pair_impl(a: impl Display, b: impl Display) -> String {
    format!("{a} and {b}")
}

// Each parameter can have its own bounds; `where` keeps the signature readable.
fn both<A, B>(shown: A, debugged: B) -> String
where
    A: Display,
    B: Debug,
{
    format!("{shown} and {debugged:?}")
}

fn main() {
    println!("{}", report(&[3, 9, 2]));
    println!("{}", report(&["kiwi", "apple", "pear"]));
    println!("pair_impl: {}", pair_impl(1, "one"));
    println!("both: {}", both("text", vec![Some(1), None]));
}
