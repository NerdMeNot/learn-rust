fn main() {
    let dice: u8 = 3; // we know it's 1 to 6. The compiler only knows it's a u8.

    let word = match dice {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
    };
    println!("{word}");
}
