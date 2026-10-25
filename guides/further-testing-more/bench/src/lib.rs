/// Total seconds in text like "1h30m".
pub fn parse_seconds(text: &str) -> u64 {
    let mut total = 0;
    let mut n = 0;
    for c in text.chars() {
        match c {
            '0'..='9' => n = n * 10 + u64::from(c as u8 - b'0'),
            's' => total += std::mem::take(&mut n),
            'm' => total += std::mem::take(&mut n) * 60,
            'h' => total += std::mem::take(&mut n) * 3600,
            _ => {}
        }
    }
    total
}
