fn main() {
    let args = std::env::args().count() as u8; // 1: known only at run time
    let a: u8 = 199 + args;
    let b: u8 = 100;
    println!("checked: {:?}", a.checked_add(b));
    println!("wrapping: {}", a.wrapping_add(b));
    println!("widened: {}", u16::from(a) + u16::from(b));
    println!("plain: {}", a + b);
}
