fn main() {
    let big: i32 = 300;
    println!("{}", big as u8); // `as` changes the value
    println!("{:?}", u8::try_from(big)); // the checked way

    // Text becomes a number only through a parse that can fail.
    println!("{:?}", "300".parse::<u8>());
    println!("{:?}", "ana".parse::<u8>());
}
