trait Vehicle {
    fn start(&self) -> String;
}

trait Appliance {
    fn start(&self) -> String;
    fn make() -> String; // no self: an associated function
}

struct Car;

impl Car {
    fn start(&self) -> String {
        String::from("Car::start (its own method)")
    }
}

impl Vehicle for Car {
    fn start(&self) -> String {
        String::from("Vehicle::start")
    }
}

impl Appliance for Car {
    fn start(&self) -> String {
        String::from("Appliance::start")
    }

    fn make() -> String {
        String::from("Appliance::make")
    }
}

fn main() {
    let car = Car;
    println!("car.start(): {}", car.start());
    println!("Vehicle::start(&car): {}", Vehicle::start(&car));
    println!("Appliance::start(&car): {}", Appliance::start(&car));
    println!("<Car as Appliance>::make(): {}", <Car as Appliance>::make());
}
