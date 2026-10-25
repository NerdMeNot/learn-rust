/// Adds two numbers.
///
/// ```
/// assert_eq!(hello::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_small_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
