fn main() {
    // Text to a number: parse. It returns a Result, because the text might not be one.
    let n: i32 = "42".parse().unwrap();
    let price = "9.99".parse::<f64>().unwrap(); // or name the type with ::<>
    println!("n + 1 = {}, price * 2 = {}", n + 1, price * 2.0);

    match "forty-two".parse::<i32>() {
        Ok(n) => println!("got {n}"),
        Err(e) => println!("not a number: {e}"),
    }

    // A number (or anything that implements Display) to text.
    let s = 3.5.to_string();
    println!("to_string: {s:?}");
    println!(
        "format!: {:?}",
        format!("{:>8.2}|{:<6}|{:06}", 3.14159, "left", 42)
    );

    // Bytes and text. Every String is valid UTF-8, so bytes that aren't get checked.
    println!("as_bytes: {:?}", "hé".as_bytes());
    println!(
        "from_utf8 (good): {:?}",
        String::from_utf8(vec![104, 195, 169])
    );
    println!(
        "from_utf8 (bad) is_err: {}",
        String::from_utf8(vec![104, 255]).is_err()
    );
    println!(
        "from_utf8_lossy: {}",
        String::from_utf8_lossy(&[104, 255, 105])
    );

    // Reversing is a job for chars, not bytes.
    let reversed: String = "héllo".chars().rev().collect();
    println!("reversed: {reversed}");
}
