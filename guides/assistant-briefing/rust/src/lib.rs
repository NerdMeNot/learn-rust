//! Parse order lines like `apple,3,40` and add them up.

use thiserror::Error;

/// One line of an order file.
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub item: String,
    pub qty: u32,
    pub unit_cents: u64,
}

impl Order {
    pub fn cents(&self) -> u64 {
        u64::from(self.qty) * self.unit_cents
    }
}

/// What can go wrong while parsing. Line numbers count from 1.
#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("line {line}: expected `item,qty,unit_cents`, found {found:?}")]
    Shape { line: usize, found: String },
    #[error("line {line}: {field} is not a number: {value:?}")]
    Number {
        line: usize,
        field: &'static str,
        value: String,
    },
}

/// Parses every non-empty line that does not start with `#`.
pub fn parse(text: &str) -> Result<Vec<Order>, ParseError> {
    let mut orders = Vec::new();
    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let n = i + 1;
        let [item, qty, unit] = line.split(',').collect::<Vec<_>>()[..] else {
            return Err(ParseError::Shape {
                line: n,
                found: line.to_string(),
            });
        };
        let number = |field: &'static str, value: &str| ParseError::Number {
            line: n,
            field,
            value: value.to_string(),
        };
        orders.push(Order {
            item: item.to_string(),
            qty: qty.parse().map_err(|_| number("qty", qty))?,
            unit_cents: unit.parse().map_err(|_| number("unit_cents", unit))?,
        });
    }
    Ok(orders)
}

/// The sum of every order, in cents.
pub fn total_cents(orders: &[Order]) -> u64 {
    orders.iter().map(Order::cents).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_comments_and_blank_lines() {
        let orders = parse("# header\n\napple,3,40\n").unwrap();
        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].cents(), 120);
    }

    #[test]
    fn reports_the_line_of_a_bad_number() {
        let err = parse("apple,3,40\nbread,one,250\n").unwrap_err();
        assert_eq!(
            err,
            ParseError::Number {
                line: 2,
                field: "qty",
                value: "one".into()
            }
        );
    }
}
