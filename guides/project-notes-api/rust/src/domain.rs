//! The domain layer: plain types and rules.

use serde::Serialize;

/// The longest text a note may have, in characters.
pub const MAX_CHARS: usize = 280;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Note {
    pub id: i64,
    pub text: String,
}

/// Note text that passed the checks. The field is private, so the only way to
/// get a `NoteText` is [`NoteText::parse`]: a store never has to check again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteText(String);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum InvalidText {
    #[error("text is empty")]
    Empty,
    #[error("text is longer than 280 characters")]
    TooLong,
}

impl NoteText {
    pub fn parse(raw: &str) -> Result<Self, InvalidText> {
        let text = raw.trim();
        if text.is_empty() {
            return Err(InvalidText::Empty);
        }
        if text.chars().count() > MAX_CHARS {
            return Err(InvalidText::TooLong);
        }
        Ok(NoteText(text.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trims_and_keeps_the_text() {
        assert_eq!(
            NoteText::parse("  buy milk \n").unwrap().as_str(),
            "buy milk"
        );
    }

    #[test]
    fn rejects_empty_and_blank_text() {
        assert_eq!(NoteText::parse(""), Err(InvalidText::Empty));
        assert_eq!(NoteText::parse(" \t\n"), Err(InvalidText::Empty));
    }

    #[test]
    fn counts_characters_not_bytes() {
        // 280 two-byte characters are 560 bytes but still allowed; 281 are not.
        assert!(NoteText::parse(&"é".repeat(MAX_CHARS)).is_ok());
        assert_eq!(
            NoteText::parse(&"é".repeat(MAX_CHARS + 1)),
            Err(InvalidText::TooLong)
        );
    }
}
