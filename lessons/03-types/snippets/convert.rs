fn main() {
    // From: only where no value can be lost.
    let small: i32 = 7;
    let wide = i64::from(small);
    let as_float = f64::from(small);
    println!("i64::from(7i32): {wide}");
    println!("f64::from(7i32): {as_float}");
    println!("u32::from('a'): {}", u32::from('a'));
    println!("u8::from(true): {}", u8::from(true));

    // TryFrom: a Result, so a value that does not fit is an Err.
    println!("u8::try_from(200i32): {:?}", u8::try_from(200_i32));
    println!("u8::try_from(300i32): {:?}", u8::try_from(300_i32));
    println!("u8::try_from(-1i32): {:?}", u8::try_from(-1_i32));
    println!("usize::try_from(-1i32): {:?}", usize::try_from(-1_i32));
    println!("i8::try_from(-128i64): {:?}", i8::try_from(-128_i64));
    println!("char::try_from(65u32): {:?}", char::try_from(65_u32));
    println!(
        "char::try_from(0xD800u32): {:?}",
        char::try_from(0xD800_u32).is_err()
    );

    // .ok() turns the Result into an Option: Ok(v) is Some(v), any Err is None.
    println!("u8::try_from(200i32).ok(): {:?}", u8::try_from(200_i32).ok());
    println!("u8::try_from(300i32).ok(): {:?}", u8::try_from(300_i32).ok());

    // try_into is the same call from the other side; the target type says which.
    let index: i32 = 2;
    let as_usize: usize = index.try_into().unwrap();
    println!("2i32 into usize: {as_usize}");
}
