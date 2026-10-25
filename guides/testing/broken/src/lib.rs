/// Convert Celsius to Fahrenheit.
///
/// A code block in a doc comment is a doc test: `cargo test` compiles and runs it.
///
/// ```
/// assert_eq!(temperature::to_fahrenheit(0.0), 32.0);
/// ```
pub fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 23.0
}

/// Parse text like "21C" or "70F" into Celsius.
///
/// ```
/// use temperature::parse_celsius;
///
/// assert_eq!(parse_celsius("70F"), Ok(21.11111111111111));
/// assert!(parse_celsius("hot").is_err());
/// ```
pub fn parse_celsius(text: &str) -> Result<f64, String> {
    // The unit is the last character, not the last byte: "21°" must be an error,
    // not a panic from slicing through the middle of a two-byte character.
    let Some(unit) = text.chars().last() else {
        return Err("empty input".to_string());
    };
    let number = &text[..text.len() - unit.len_utf8()];
    let value: f64 = number
        .parse()
        .map_err(|_| format!("not a number: {number:?}"))?;
    match unit {
        'C' => Ok(value),
        'F' => Ok((value - 32.0) * 5.0 / 9.0),
        _ => Err(format!("unknown unit: {unit:?}")),
    }
}

// Unit tests live next to the code. #[cfg(test)] leaves them out of normal builds.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boiling_point() {
        assert_eq!(to_fahrenheit(100.0), 212.0);
    }

    #[test]
    fn parses_fahrenheit() {
        let parsed = parse_celsius("212F");
        println!("212F parsed to {parsed:?}");
        assert_eq!(parsed, Ok(100.0));
    }

    #[test]
    fn rejects_unknown_unit() {
        assert!(parse_celsius("20K").is_err());
        assert!(parse_celsius("21°").is_err());
    }
}
