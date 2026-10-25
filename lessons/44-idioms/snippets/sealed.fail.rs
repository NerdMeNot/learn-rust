mod shapes {
    mod private {
        pub trait Sealed {}
    }

    pub trait Shape: private::Sealed {
        fn sides(&self) -> u32;
    }
}

struct Circle;

impl shapes::Shape for Circle {
    fn sides(&self) -> u32 {
        0
    }
}

fn main() {
    let _circle = Circle;
}
