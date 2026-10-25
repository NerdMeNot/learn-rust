fn main() {
    // `match` is not a loop and not a switch: `break` here leaves the `for`
    for n in 1..=5 {
        match n {
            3 => break,
            _ => println!("n: {n}"),
        }
    }
    println!("after the loop");
}
