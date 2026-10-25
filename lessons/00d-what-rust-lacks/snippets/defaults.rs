#[derive(Default)]
struct Window {
    width: u32,
    height: u32,
    border: bool,
}

fn describe(w: &Window) -> String {
    format!("{}x{} border={}", w.width, w.height, w.border)
}

fn scaled(width: u32, factor: Option<u32>) -> u32 {
    width * factor.unwrap_or(1)
}

fn main() {
    let sized = Window {
        width: 80,
        height: 24,
        ..Default::default()
    };
    let bordered = Window {
        border: true,
        ..Default::default()
    };
    println!("sized: {}", describe(&sized));
    println!("bordered: {}", describe(&bordered));
    println!("scaled: {} {}", scaled(3, None), scaled(3, Some(4)));
}
