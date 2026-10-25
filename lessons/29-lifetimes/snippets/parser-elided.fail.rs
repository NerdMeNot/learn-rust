struct Parser<'a> {
    rest: &'a str,
}

impl<'a> Parser<'a> {
    // No 'a on the result: by rule 3 it borrows from `&mut self`.
    fn next_word(&mut self) -> Option<&str> {
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
    let mut parser = Parser { rest: &text };
    let a = parser.next_word();
    let b = parser.next_word();
    println!("a: {a:?}, b: {b:?}");
}
