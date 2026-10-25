fn main() {
    let hello = String::from("Hello, ");
    let world = String::from("world");

    // + takes the left String by value and appends to its buffer. The right side
    // is only read, so it is borrowed with &.
    let greeting = hello + &world;
    println!("{greeting}");
    println!("{world}"); // still fine: world was only borrowed

    // For more than two pieces, format! reads better and borrows everything.
    let a = String::from("tic");
    let b = String::from("tac");
    let c = String::from("toe");
    let all = format!("{a}-{b}-{c}");
    println!("{all} ({a} is still here)");
}
