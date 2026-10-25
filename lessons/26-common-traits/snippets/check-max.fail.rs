// max and min are Ord methods: a type with only PartialOrd does not have them. Not shown on the page.
#[derive(Debug, PartialEq, PartialOrd)]
struct Reading(f64);

fn main() {
    let a = Reading(1.0);
    let b = Reading(2.0);
    println!("{}", a < b);
    println!("{:?}", a.max(b));
}
