fn main() {
    // One char can become several: to_uppercase returns an iterator of chars.
    let up = 'ß'.to_uppercase();
    println!("'ß'.to_uppercase(): {up} ({} chars)", up.len());
    println!("'ß'.to_ascii_uppercase(): {}", 'ß'.to_ascii_uppercase());
    println!("\"straße\".to_uppercase(): {}", "straße".to_uppercase());

    // The str method knows that a Greek capital sigma ends a word as ς.
    println!("\"ΑΣ\".to_lowercase():     {}", "ΑΣ".to_lowercase());
    let per_char: String = "ΑΣ".chars().flat_map(char::to_lowercase).collect();
    println!("char by char:            {per_char}");

    // No locale: "I" lowercases to "i" everywhere. Turkish wants a dotless ı.
    println!("\"I\".to_lowercase():      {}", "I".to_lowercase());
    println!(
        "\"İ\".to_lowercase() chars: {:?}",
        "İ".to_lowercase().chars().collect::<Vec<_>>()
    );

    // Comparing without regard to case: ASCII only, or lowercase both sides.
    println!(
        "ascii-ignore-case(\"Rust\", \"RUST\"):    {}",
        "Rust".eq_ignore_ascii_case("RUST")
    );
    println!(
        "ascii-ignore-case(\"É\", \"é\"):          {}",
        "É".eq_ignore_ascii_case("é")
    );
    println!(
        "lowercase both(\"É\", \"é\"):             {}",
        "É".to_lowercase() == "é".to_lowercase()
    );
    println!(
        "lowercase both(\"STRASSE\", \"straße\"):  {}",
        "STRASSE".to_lowercase() == "straße".to_lowercase()
    );
}
