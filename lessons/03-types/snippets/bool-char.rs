fn main() {
    println!("size of bool: {}", size_of::<bool>());
    println!("size of char: {}", size_of::<char>());
    println!("char::MAX: {:?} = U+{:X}", char::MAX, char::MAX as u32);
    println!("char::from_u32(0x41): {:?}", char::from_u32(0x41));
    println!("char::from_u32(0xD800): {:?}", char::from_u32(0xD800));
    println!("char::from_u32(1114112): {:?}", char::from_u32(1_114_112));

    // Three characters, three lengths in UTF-8.
    for c in ['a', 'é', '€', '🦀'] {
        println!(
            "{c} is U+{:04X}, {} byte(s) in UTF-8",
            c as u32,
            c.len_utf8()
        );
    }

    // A u8 is a number that can be written as a byte literal.
    let letter = b'a';
    println!("b'a': {letter}");
    println!("b'a' + 1: {}", letter + 1);
    println!("as a char: {}", (letter + 1) as char);
    println!("'7'.to_digit(10): {:?}", '7'.to_digit(10));
    println!("'x'.to_digit(10): {:?}", 'x'.to_digit(10));
    println!("char::from_digit(7, 10): {:?}", char::from_digit(7, 10));
    println!("'€' as u8: {}", '€' as u8);
}
