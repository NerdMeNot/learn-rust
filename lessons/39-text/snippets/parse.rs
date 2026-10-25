fn main() {
    // What parse::<i32> accepts and refuses. The whole text must be the number.
    for text in ["42", "+42", "-0", " 42", "42 ", "1_000", "4.0", "0x1f", ""] {
        println!("{text:?}: {:?}", text.parse::<i32>());
    }

    // The target type decides what fits.
    println!(
        "200 as i16, u8, i8: {:?} {:?} {:?}",
        "200".parse::<i16>(),
        "200".parse::<u8>(),
        "200".parse::<i8>()
    );

    // Out of range is an error for the target type, not a wrap-around.
    println!("300 as u8:  {:?}", "300".parse::<u8>());
    println!("-1 as u8:   {:?}", "-1".parse::<u8>());
    println!(
        "3000000000 as i32, i64: {:?} {:?}",
        "3000000000".parse::<i32>(),
        "3000000000".parse::<i64>()
    );
    println!("-129 as i8: {:?}", "-129".parse::<i8>());

    // The error type has a message (Display) and a kind you can match on.
    let err = "x1".parse::<i32>().unwrap_err();
    println!("message: {err}");
    println!("kind:    {:?}", err.kind());

    // Other FromStr types.
    println!(
        "f64: {:?} {:?} {:?} {:?}",
        "1e3".parse::<f64>(),
        ".5".parse::<f64>(),
        "NaN".parse::<f64>(),
        "inf".parse::<f64>()
    );
    println!("f64 with a comma: {:?}", "1,5".parse::<f64>());
    println!(
        "bool: {:?} {:?}",
        "true".parse::<bool>(),
        "True".parse::<bool>()
    );
    println!("char: {:?} {:?}", "x".parse::<char>(), "xy".parse::<char>());
    println!("radix 16: {:?}", i32::from_str_radix("ff", 16));
    println!("Arabic-Indic three: {:?}", "٣".parse::<i32>());
}
