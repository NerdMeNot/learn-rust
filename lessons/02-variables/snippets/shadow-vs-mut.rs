fn main() {
    let mut a = 1;
    let b = 1;
    println!("before: a = {a}, b = {b}");
    {
        a = 10; // changes the one variable
        let b = 10; // a new variable, gone when the block ends
        println!("inside: a = {a}, b = {b}");
    }
    println!("after: a = {a}, b = {b}");
}
