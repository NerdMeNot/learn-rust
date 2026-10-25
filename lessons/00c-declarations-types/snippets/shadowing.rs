fn main() {
    let spaces = "   ";
    let spaces = spaces.len(); // a new variable that reuses the name, with a new type
    println!("spaces: {spaces}");

    let x = 1;
    {
        let x = x * 10; // shadows only inside this block
        println!("inner x: {x}");
    }
    println!("outer x: {x}");
}
