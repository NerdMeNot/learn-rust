// A sealed trait: public, but only this crate can implement it.
mod shapes {
    mod private {
        pub trait Sealed {}
    }

    pub trait Shape: private::Sealed {
        fn sides(&self) -> u32;
    }

    pub struct Square;
    pub struct Triangle;

    impl private::Sealed for Square {}
    impl private::Sealed for Triangle {}

    impl Shape for Square {
        fn sides(&self) -> u32 {
            4
        }
    }

    impl Shape for Triangle {
        fn sides(&self) -> u32 {
            3
        }
    }
}

use shapes::{Shape, Square, Triangle};

fn main() {
    assert_eq!(Square.sides(), 4);
    assert_eq!(Triangle.sides(), 3);
    println!("sealed trait: the crate's own types implement it");
}
