fn main() {
    let x: i8 = 5;
    println!("x:             {:08b}", x);
    println!("-x:            {:08b}", -x);
    println!("!x + 1:        {:08b}", !x + 1);
    println!("-1_i8:         {:08b}", -1_i8);
    println!("i8::MIN:       {:08b} = {}", i8::MIN, i8::MIN);
    println!("i8::MAX:       {:08b} = {}", i8::MAX, i8::MAX);

    // The same bits read as unsigned, and the other way round.
    println!("-x as u8:      {:08b} = {}", -x as u8, -x as u8);
    println!("200_u8 as i8:  {:08b} = {}", 200_u8 as i8, 200_u8 as i8);

    // Adding one to the largest value carries into the sign bit.
    println!("i8::MAX.wrapping_add(1): {}", i8::MAX.wrapping_add(1));
    println!("255u8.wrapping_add(1):   {}", 255_u8.wrapping_add(1));

    // Narrowing keeps the low bits; hex shows which.
    let wide: i32 = 0xA1B2C;
    println!("wide:          {:#x}", wide);
    println!("as u16:        {:#x}", wide as u16);
    println!("as u8:         {:#x}", wide as u8);
    println!("300:           {:#b}", 300_i32);
    println!("300_i32 as u8: {:#010b} -> {}", 300_i32 as u8, 300_i32 as u8);
}
