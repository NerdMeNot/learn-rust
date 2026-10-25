trait Vehicle {
    fn start(&self) -> String {
        String::from("Vehicle::start")
    }
}

trait Appliance {
    fn start(&self) -> String {
        String::from("Appliance::start")
    }
}

struct Car;

// Both impls are accepted, even though both traits have a default `start`.
impl Vehicle for Car {}
impl Appliance for Car {}

fn main() {
    println!("{}", Car.start());
}
