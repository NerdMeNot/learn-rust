struct Parser<'a> {
    rest: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Parser<'a> {
        Parser { rest: input }
    }

    // The word borrows from the text (`'a`), not from the parser.
    fn next_word(&mut self) -> Option<&'a str> {
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
    let mut parser = Parser::new(&text);
    let a = parser.next_word();
    let b = parser.next_word(); // `a` is still alive here
    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("c: {:?}", parser.next_word());
    println!("d: {:?}", parser.next_word());
}
