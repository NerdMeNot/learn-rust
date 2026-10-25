fn main() {
    // Result keeps the reason a parse failed. Option has no room for one.
    println!("parse '42': {:?}", "42".parse::<u32>());
    println!("parse '4x2': {:?}", "4x2".parse::<u32>());

    // .ok() turns a Result into an Option and drops the reason.
    println!("parse '42', then ok(): {:?}", "42".parse::<u32>().ok());
    println!("parse '4x2', then ok(): {:?}", "4x2".parse::<u32>().ok());

    // ok_or goes the other way: it gives a None a reason.
    println!("None.ok_or(...): {:?}", None::<u32>.ok_or("no value given"));
}
