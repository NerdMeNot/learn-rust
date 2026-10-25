fn main() {
    let big: i32 = 300;
    let neg: i32 = -1;
    let x: f64 = -2.9;

    println!("big as u8: {}", big as u8);
    println!("-1 as u8: {}", neg as u8);
    println!("-1 as u32: {}", neg as u32);
    println!("200u8 as i8: {}", 200u8 as i8);
    println!("-2.9 as i32: {}", x as i32);
    println!("300.0 as u8: {}", 300.0f64 as u8);
    println!("-5.0 as u8: {}", -5.0f64 as u8);
    println!("NaN as i32: {}", f64::NAN as i32);
    println!("'a' as u32: {}", 'a' as u32);
    println!("97u8 as char: {}", 97u8 as char);
    println!("true as i32: {}", true as i32);
    println!("i64::from(7i32): {}", i64::from(7i32));
    println!("u8::try_from(300i32): {:?}", u8::try_from(300i32));
}
