/// Kelvin from Celsius. Panics below absolute zero.
pub fn to_kelvin(celsius: f64) -> f64 {
    assert!(celsius >= -273.15, "below absolute zero: {celsius}");
    celsius + 273.15
}

#[cfg(test)]
mod tests {
    use super::*;

    // Passes only if the code panics with a message containing this text.
    #[test]
    #[should_panic(expected = "below absolute zero")]
    fn rejects_below_absolute_zero() {
        to_kelvin(-300.0);
    }

    // A test may return a Result, so `?` works inside it. Err fails the test.
    #[test]
    fn freezing_point() -> Result<(), String> {
        let text = "0";
        let celsius: f64 = text.parse().map_err(|e| format!("{e}"))?;
        assert_eq!(to_kelvin(celsius), 273.15);
        Ok(())
    }

    // Skipped by a plain `cargo test`; `cargo test -- --ignored` runs only these.
    #[test]
    #[ignore = "pretend this one is slow"]
    fn absolute_zero() {
        assert_eq!(to_kelvin(-273.15), 0.0);
    }

    // Floating point: compare with a tolerance unless you know the result is exact.
    #[test]
    fn floats_need_a_tolerance() {
        assert_ne!(0.1 + 0.2, 0.3);
        assert!((0.1_f64 + 0.2 - 0.3).abs() < 1e-9);
    }
}
