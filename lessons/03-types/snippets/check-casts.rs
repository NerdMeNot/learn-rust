// Backs the cast table on the page: every row is asserted here.
fn main() {
    // Narrowing keeps the low bits.
    assert_eq!(300_i32 as u8, 44);
    assert_eq!(0x1234_5678_i32 as u8, 0x78);
    assert_eq!(70_000_i32 as i16, 4464);
    assert_eq!(u64::MAX as u32, u32::MAX);
    // Same size, different sign: the bits stay.
    assert_eq!(-1_i32 as u32, 4_294_967_295);
    assert_eq!(200_u8 as i8, -56);
    assert_eq!(u32::MAX as i32, -1);
    // Widening copies the value; a signed source is sign-extended first.
    assert_eq!(-1_i8 as i32, -1);
    assert_eq!(255_u8 as i32, 255);
    assert_eq!(-1_i8 as u32, 4_294_967_295);
    assert_eq!(-1_i8 as u8 as u32, 255);
    assert_eq!(-1_i32 as i64, -1);
    assert_eq!(-1_i32 as u64, 18_446_744_073_709_551_615);
    // bool and char.
    assert_eq!(true as i32, 1);
    assert_eq!('a' as u32, 97);
    assert_eq!('€' as u32, 8364);
    assert_eq!('€' as u8, 0xAC);
    assert_eq!(97_u8 as char, 'a');
    // Floats.
    assert_eq!(-2.9_f64 as i32, -2);
    assert_eq!(1e10_f64 as i32, i32::MAX);
    assert_eq!(-1e10_f64 as i32, i32::MIN);
    assert_eq!(300.7_f64 as u8, 255);
    assert_eq!(-5.0_f64 as u8, 0);
    assert_eq!(f64::NAN as i32, 0);
    assert_eq!(f64::INFINITY as i64, i64::MAX);
    assert_eq!(16_777_217_i32 as f32, 16_777_216.0);
    assert_eq!(16_777_217_i32 as f64, 16_777_217.0);
    assert_eq!(1.0e40_f64 as f32, f32::INFINITY);
    // An f32 keeps 24 bits of the number, so 2^24 + 1 (25 bits) is the first integer it skips.
    assert_eq!(f32::MANTISSA_DIGITS, 24);
    println!("all casts checked");
}
