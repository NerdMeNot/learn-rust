fn main() {
    println!("\"42\": {:?}", "42".parse::<i32>());
    println!("\"-5\" to u8: {:?}", "-5".parse::<u8>());
    println!("\"300\" to u8: {:?}", "300".parse::<u8>());
    println!("\"4x\": {:?}", "4x".parse::<i32>());
    println!("\"\": {:?}", "".parse::<i32>());
    println!("\" 42\": {:?}", " 42".parse::<i32>());
    println!("\" 42\".trim(): {:?}", " 42".trim().parse::<i32>());
    println!("\"+7\": {:?}", "+7".parse::<i32>());
    println!("\"2.5\" to f64: {:?}", "2.5".parse::<f64>());
    println!("\"2.5\" to i32: {:?}", "2.5".parse::<i32>());
    println!("\"NaN\" to f64: {:?}", "NaN".parse::<f64>());
    println!("\"true\": {:?}", "true".parse::<bool>());

    // The type can come from where the value goes.
    let n: u16 = "8080".parse().unwrap();
    println!("port: {n}");
}
