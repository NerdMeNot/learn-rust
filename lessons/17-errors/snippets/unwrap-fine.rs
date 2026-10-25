use std::net::Ipv4Addr;

fn main() {
    // The text is a literal in this file, and a valid address. If someone mistypes
    // it later, the program stops at once and the message says what was meant.
    let home: Ipv4Addr = "127.0.0.1".parse().expect("the literal is a valid address");
    println!("home: {home}");

    // A fallback chosen on purpose is not a crash waiting to happen.
    let width: u32 = "wide".parse().unwrap_or(80);
    println!("width: {width}");
}
