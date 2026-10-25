// This program only measures sizes, so it never builds the variants.
#![allow(dead_code)]

use std::mem::{align_of, size_of};

enum Plain {
    A,
    B,
    C,
}

enum Small {
    Byte(u8),
    Nothing,
}

enum Shape {
    Circle(f64),
    Rect(f64, f64),
}

enum Command {
    Quit,
    Say(String),
    Paint(u8, u8, u8),
}

enum Big {
    Quit,
    Say(String),
    Paint(u64, u64, u64),
}

fn main() {
    println!("Plain (3 variants, no data): {} byte", size_of::<Plain>());
    println!("Small (u8 or nothing):       {} bytes", size_of::<Small>());
    println!("Shape (f64, or two f64):     {} bytes", size_of::<Shape>());
    println!(
        "  its biggest variant data:  {} bytes",
        size_of::<(f64, f64)>()
    );
    println!("  its alignment:             {} bytes", align_of::<Shape>());
    println!(
        "Command (String, 3 u8, none): {} bytes",
        size_of::<Command>()
    );
    println!("  a String alone:            {} bytes", size_of::<String>());
    println!("Big (String, 3 u64, none):   {} bytes", size_of::<Big>());
}
