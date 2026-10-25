use std::fmt::{self, Display, Formatter};

struct List<'a, T>(&'a [T]);

impl<T: Display> Display for List<'_, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                f.write_str(", ")?;
            }
            item.fmt(f)?; // every item gets the caller's spec
        }
        Ok(())
    }
}

fn main() {
    println!("[{}]", List(&[1.5, 2.25, 3.0]));
    println!("[{:.1}]", List(&[1.5, 2.26, 3.0]));
    println!("[{:>4}]", List(&[1, 22, 333]));
    println!("[{}]", List::<i32>(&[]));
}
