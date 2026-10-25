struct Words<'a> {
    rest: &'a str,
}

impl<'a> Iterator for Words<'a> {
    // Every item borrows from the text, not from the Words that found it.
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        let rest = self.rest.trim_start();
        if rest.is_empty() {
            return None;
        }
        let (word, tail) = rest.split_once(' ').unwrap_or((rest, ""));
        self.rest = tail;
        Some(word)
    }
}

fn main() {
    let text = String::from("lifetimes are regions");
    let words: Vec<&str> = Words { rest: &text }.collect();
    println!("words: {words:?}");
}
