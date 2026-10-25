struct Celsius(f64);

impl Celsius {
    const ABSOLUTE_ZERO: f64 = -273.15;

    /// An associated function: no `self`. Celsius = (F - 32) * 5 / 9.
    fn from_fahrenheit(f: f64) -> Self {
        todo!("Self((f - 32.0) * 5.0 / 9.0)")
    }

    /// Reads the temperature. Fahrenheit = C * 9 / 5 + 32.
    fn to_fahrenheit(&self) -> f64 {
        todo!("self.0 * 9.0 / 5.0 + 32.0")
    }

    /// Lowers the temperature, but never below ABSOLUTE_ZERO.
    fn cool(&mut self, degrees: f64) {
        todo!("subtract, then f64::max with Self::ABSOLUTE_ZERO")
    }

    /// Takes both temperatures and gives back the colder one.
    fn colder(self, other: Celsius) -> Celsius {
        todo!("compare self.0 and other.0")
    }
}

fn main() {
    assert_eq!(Celsius::from_fahrenheit(212.0).0, 100.0);
    assert_eq!(Celsius::from_fahrenheit(-40.0).0, -40.0);
    assert_eq!(Celsius(100.0).to_fahrenheit(), 212.0);
    assert_eq!(Celsius(0.0).to_fahrenheit(), 32.0);

    let mut c = Celsius(20.0);
    c.cool(5.0);
    assert_eq!(c.0, 15.0);
    c.cool(1000.0);
    assert_eq!(
        c.0,
        Celsius::ABSOLUTE_ZERO,
        "nothing is colder than absolute zero"
    );

    let a = Celsius(3.0);
    let b = Celsius(-4.5);
    assert_eq!(a.colder(b).0, -4.5);

    println!("all checks passed");
}
