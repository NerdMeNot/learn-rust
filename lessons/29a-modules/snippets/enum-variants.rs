mod light {
    // The variants are not marked `pub`: they take the enum's visibility.
    pub enum Color {
        Red,
        Green,
    }

    pub struct Lamp {
        pub on: bool, // struct fields do not work that way: each needs its own `pub`
    }
}

fn main() {
    for c in [light::Color::Red, light::Color::Green] {
        let name = match c {
            light::Color::Red => "red",
            light::Color::Green => "green",
        };
        println!("color: {name}");
    }
    println!("lamp on: {}", light::Lamp { on: true }.on);
}
