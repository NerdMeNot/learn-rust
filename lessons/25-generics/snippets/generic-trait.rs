// A trait can take a type parameter too.
trait Convert<T> {
    fn convert(&self) -> T;
}

struct Celsius(f64);

impl Convert<f64> for Celsius {
    fn convert(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Convert<String> for Celsius {
    fn convert(&self) -> String {
        format!("{} degrees C", self.0)
    }
}

fn main() {
    let temp = Celsius(100.0);
    // The type you ask for picks the impl.
    let fahrenheit: f64 = temp.convert();
    let label: String = temp.convert();
    println!("as f64: {fahrenheit}");
    println!("as String: {label}");
}
