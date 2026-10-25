enum Light {
    Red,
    Yellow,
    Green,
}

fn main() {
    for light in [Light::Red, Light::Yellow, Light::Green] {
        let action = match light {
            Light::Red => "stop",
            _ => "go",
        };
        println!("{action}");
    }
}
