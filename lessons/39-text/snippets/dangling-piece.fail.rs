fn main() {
    // The String is a temporary: it is dropped at the end of this statement,
    // but the piece still points into it.
    let piece = String::from("alpha,beta").split(',').next().unwrap();
    println!("piece: {piece}");
}
