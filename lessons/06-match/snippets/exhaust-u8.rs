fn main() {
    for byte in [10u8, 65, 127, 200] {
        let class = match byte {
            0..=31 => "control",
            32..=126 => "printable",
            127 => "delete",
            128..=u8::MAX => "high",
        };
        println!("{byte} -> {class}");
    }
}
