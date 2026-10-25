enum Color {
    Red,
    Green,
}

fn main() {
    let c = Color::Red;
    if c == Color::Green {
        println!("green");
    }
    println!("{c:?}");
}
