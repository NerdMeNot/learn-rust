// Backs the lesson's remark: a local type among the trait's own parameters is allowed.
struct Meters(f64);

impl From<Meters> for f64 {
    fn from(m: Meters) -> f64 {
        m.0
    }
}

fn main() {
    let x: f64 = Meters(2.5).into();
    assert_eq!(x, 2.5);
}
