fn main() {
    // Unicode classes versus ASCII classes. U+0663 is the Arabic-Indic digit three.
    let three = '٣';
    println!("is_numeric:      {}", three.is_numeric());
    println!("is_ascii_digit:  {}", three.is_ascii_digit());
    println!("to_digit(10):    {:?}", three.to_digit(10));
    println!("é is_alphabetic: {}", 'é'.is_alphabetic());
    println!("é is_ascii_alphabetic: {}", 'é'.is_ascii_alphabetic());

    // to_digit takes a radix up to 36. from_digit goes the other way.
    println!("'7'.to_digit(10): {:?}", '7'.to_digit(10));
    println!("'f'.to_digit(16): {:?}", 'f'.to_digit(16));
    println!("'z'.to_digit(36): {:?}", 'z'.to_digit(36));
    println!("'f'.to_digit(10): {:?}", 'f'.to_digit(10));
    println!("from_digit(7, 10): {:?}", char::from_digit(7, 10));

    // Between chars and numbers. A u8 is always a valid char; a u32 may not be.
    println!("'A' as u32:    {}", 'A' as u32);
    println!("97u8 as char:  {}", 97u8 as char);
    println!("from_u32(233): {:?}", char::from_u32(233));
    println!("from_u32(0xD800): {:?}", char::from_u32(0xD800));
}
