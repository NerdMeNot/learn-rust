use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Meters(f64);

// `Add` is declared `Add<Rhs = Self>`: a type parameter with a default.
// Writing `Add` alone means `Add<Meters>`.
impl Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

// The same type can implement it again for another right-hand side.
impl Add<f64> for Meters {
    type Output = Meters;
    fn add(self, other: f64) -> Meters {
        Meters(self.0 + other)
    }
}

fn main() {
    println!("Meters + Meters: {:?}", Meters(1.0) + Meters(2.0));
    println!("Meters + f64: {:?}", Meters(1.0) + 0.5);
}
