/// The first letter of each word, upper-cased: "Ada Lovelace" -> "AL".
/// Words are separated by any amount of whitespace.
pub fn initials(name: &str) -> String {
    todo!("initials of {name:?}")
}

#[cfg(test)]
mod tests {
    use super::initials;

    #[test]
    fn two_words() {
        assert_eq!(initials("Ada Lovelace"), "AL");
    }

    #[test]
    fn extra_spaces() {
        assert_eq!(initials("  grace   hopper "), "GH");
    }

    #[test]
    fn empty() {
        assert_eq!(initials(""), "");
    }

    #[test]
    fn not_ascii() {
        assert_eq!(initials("élodie durand"), "ÉD");
    }
}
