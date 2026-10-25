trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> String;

    /// Default: "<name> with area <area>", the area with two decimals.
    fn describe(&self) -> String {
        todo!("format self.name() and self.area() with {{:.2}}")
    }
}

struct Square(f64);
struct Circle(f64);
struct Triangle {
    base: f64,
    height: f64,
}

// Area of a square is side * side; of a circle, PI * radius * radius.
// Names are "square" and "circle".
impl Shape for Square {
    fn area(&self) -> f64 {
        todo!("side * side")
    }
    fn name(&self) -> String {
        todo!("the name")
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        todo!("PI * radius * radius, with std::f64::consts::PI")
    }
    fn name(&self) -> String {
        todo!("the name")
    }
}

// Triangle gives only the two required methods and has no describe().
// Its describe() must still work, so the default belongs in the trait.
impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
    fn name(&self) -> String {
        String::from("triangle")
    }
}

/// The position of the item with the largest area, or None for an empty slice.
/// (If two tie, either position is fine.)
fn biggest<T: Shape>(items: &[T]) -> Option<usize> {
    todo!("compare areas with total_cmp; return the winner's position")
}

/// The total area of a mixed list: a Square next to a Circle.
/// `&dyn Shape` is a reference to any type that implements Shape.
fn total_area(items: &[&dyn Shape]) -> f64 {
    todo!("a slice of &dyn Shape holds different types; add up the areas")
}

fn main() {
    assert_eq!(Square(3.0).describe(), "square with area 9.00");
    assert_eq!(Circle(1.0).describe(), "circle with area 3.14");
    // Triangle never wrote describe(): the default from the trait is used.
    let t = Triangle {
        base: 3.0,
        height: 5.0,
    };
    assert_eq!(t.describe(), "triangle with area 7.50");
    // `biggest` takes a slice of ONE type (T). A mixed list needs `dyn`.
    assert_eq!(biggest(&[Square(1.0), Square(3.0), Square(2.0)]), Some(1));
    assert_eq!(biggest(&[Circle(0.5), Circle(2.0)]), Some(1));
    assert_eq!(biggest(&[Circle(2.0), Circle(0.5)]), Some(0));
    // No items, no biggest.
    assert_eq!(biggest::<Square>(&[]), None);
    let mixed: [&dyn Shape; 3] = [&Square(2.0), &Circle(1.0), &t];
    let expected = 4.0 + std::f64::consts::PI + 7.5;
    assert!((total_area(&mixed) - expected).abs() < 1e-9);
    // The default method works through `dyn` too.
    assert_eq!(mixed[2].describe(), "triangle with area 7.50");
    assert_eq!(total_area(&[]), 0.0);
    println!("all checks passed");
}
