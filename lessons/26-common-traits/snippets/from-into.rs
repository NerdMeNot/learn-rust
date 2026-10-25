#[derive(Debug)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

// Write From once ...
impl From<(u8, u8, u8)> for Rgb {
    fn from(t: (u8, u8, u8)) -> Rgb {
        Rgb {
            r: t.0,
            g: t.1,
            b: t.2,
        }
    }
}

impl From<Rgb> for u32 {
    fn from(c: Rgb) -> u32 {
        ((c.r as u32) << 16) | ((c.g as u32) << 8) | c.b as u32
    }
}

// ... and Into comes with it. A parameter can ask for "anything that converts to String".
fn greet(name: impl Into<String>) -> String {
    let name: String = name.into();
    format!("hello, {name}")
}

fn main() {
    let a = Rgb::from((255, 128, 0));
    let b: Rgb = (0, 0, 255).into();
    println!("{a:?} {b:?}");

    let packed: u32 = a.into();
    println!("packed: {packed:06x}");

    println!("{}", greet("ana"));
    println!("{}", greet(String::from("bo")));

    let wide = i64::from(7_i32);
    let letter = char::from(65_u8);
    let code = u32::from('A');
    println!("{wide} {letter} {code}");
}
