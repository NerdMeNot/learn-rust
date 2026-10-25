use std::fmt;

struct Matrix([[i32; 2]; 2]);

impl Matrix {
    // A value that prints a row, without a new type and impl for it.
    fn row(&self, i: usize) -> impl fmt::Display + '_ {
        fmt::from_fn(move |f| write!(f, "{} {}", self.0[i][0], self.0[i][1]))
    }
}

fn main() {
    let m = Matrix([[1, 2], [3, 4]]);
    println!("row 0: {}", m.row(0));
    println!("row 1: {}", m.row(1));
    println!("as string: {:?}", m.row(1).to_string());
}
