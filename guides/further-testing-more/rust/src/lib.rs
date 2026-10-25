//! Small helpers for a time-tracking tool.

pub mod cache;
pub mod session;
pub mod store;

/// Why a duration like `"1h30m"` could not be read.
#[derive(Debug, PartialEq)]
pub enum ParseError {
    Empty,
    NoNumber,
    MissingUnit,
    UnknownUnit(char),
    TooBig,
}

// Private on purpose: unit tests inside this file can call it, the tests/ folder cannot.
fn unit_seconds(unit: char) -> Option<u64> {
    match unit {
        's' => Some(1),
        'm' => Some(60),
        'h' => Some(3600),
        _ => None,
    }
}

/// Reads text like `"1h30m"` or `"90s"` and returns the number of seconds.
///
/// ```
/// assert_eq!(timekeep::parse_duration("1h30m"), Ok(5400));
/// ```
///
/// A number without a unit is an error. The `?` in a doc test needs a function that returns
/// a `Result`; hidden lines (they start with `# `) supply it:
///
/// ```
/// # fn main() -> Result<(), timekeep::ParseError> {
/// let seconds = timekeep::parse_duration("2m")?;
/// assert_eq!(seconds, 120);
/// # Ok(())
/// # }
/// ```
///
/// ```
/// assert!(timekeep::parse_duration("15").is_err());
/// ```
pub fn parse_duration(text: &str) -> Result<u64, ParseError> {
    if text.is_empty() {
        return Err(ParseError::Empty);
    }
    let mut total: u64 = 0;
    let mut digits = String::new();
    for c in text.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
            continue;
        }
        let per = unit_seconds(c).ok_or(ParseError::UnknownUnit(c))?;
        if digits.is_empty() {
            return Err(ParseError::NoNumber);
        }
        let n: u64 = digits.parse().map_err(|_| ParseError::TooBig)?;
        total = n
            .checked_mul(per)
            .and_then(|s| total.checked_add(s))
            .ok_or(ParseError::TooBig)?;
        digits.clear();
    }
    if digits.is_empty() {
        Ok(total)
    } else {
        Err(ParseError::MissingUnit)
    }
}

/// The average of some samples, or `None` when there are none.
///
/// This version adds up in `i32`, so a few large samples overflow. See `average`.
pub fn average_naive(samples: &[i32]) -> Option<i32> {
    if samples.is_empty() {
        return None;
    }
    Some(samples.iter().sum::<i32>() / samples.len() as i32)
}

/// The average of some samples, or `None` when there are none. Adds up in `i64`.
///
/// ```
/// assert_eq!(timekeep::average(&[i32::MAX, i32::MAX]), Some(i32::MAX));
/// assert_eq!(timekeep::average(&[]), None);
/// ```
pub fn average(samples: &[i32]) -> Option<i32> {
    if samples.is_empty() {
        return None;
    }
    let sum: i64 = samples.iter().map(|&s| i64::from(s)).sum();
    Some((sum / samples.len() as i64) as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Table-driven: one row per case. The loop collects every failure, so one run lists them all.
    #[test]
    fn parse_duration_table() {
        let cases: &[(&str, Result<u64, ParseError>)] = &[
            ("90s", Ok(90)),
            ("2m", Ok(120)),
            ("1h30m", Ok(5400)),
            ("1h1m1s", Ok(3661)),
            ("", Err(ParseError::Empty)),
            ("15", Err(ParseError::MissingUnit)),
            ("m", Err(ParseError::NoNumber)),
            ("5x", Err(ParseError::UnknownUnit('x'))),
            ("99999999999999999999h", Err(ParseError::TooBig)),
        ];
        let mut failures = Vec::new();
        for (input, expected) in cases {
            let got = parse_duration(input);
            if &got != expected {
                failures.push(format!("{input:?}: expected {expected:?}, got {got:?}"));
            }
        }
        assert!(
            failures.is_empty(),
            "{} case(s) failed:\n{}",
            failures.len(),
            failures.join("\n")
        );
    }

    // A unit test can call a private function: `use super::*` brings it in.
    #[test]
    fn unit_seconds_knows_three_units() {
        assert_eq!(unit_seconds('m'), Some(60));
        assert_eq!(unit_seconds('d'), None);
    }

    // Example-based tests with small numbers do not see the overflow.
    #[test]
    fn naive_average_works_for_small_numbers() {
        assert_eq!(average_naive(&[1, 2, 3]), Some(2));
        assert_eq!(average_naive(&[]), None);
    }
}
