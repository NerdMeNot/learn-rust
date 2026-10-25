#[derive(Debug, PartialEq, PartialOrd)]
struct Reading(f64);

fn main() {
    let missing = Reading(f64::NAN);
    println!("{}", missing == missing);
    println!("{:?}", Reading(1.0).partial_cmp(&missing));
    println!("{}", Reading(1.0) < missing);
    println!("{}", Reading(1.0) == Reading(1.0));
}
