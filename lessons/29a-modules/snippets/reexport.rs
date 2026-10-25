mod shapes {
    pub mod circle {
        pub struct Circle {
            pub r: f64,
        }

        pub fn area(c: &Circle) -> f64 {
            std::f64::consts::PI * c.r * c.r
        }
    }

    // A second name for the same item, one level up.
    pub use self::circle::Circle;
}

use shapes::Circle;

fn main() {
    let a: shapes::circle::Circle = Circle { r: 2.0 };
    let b: shapes::Circle = a; // the same type, reached by two paths
    println!("area: {:.2}", shapes::circle::area(&b));
}
