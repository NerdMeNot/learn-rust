use std::fmt;

struct Parser<'a> {
    rest: &'a str,
}

// `'_` means "some lifetime, and I do not need its name".
impl Parser<'_> {
    fn remaining(&self) -> usize {
        self.rest.len()
    }
}

impl fmt::Display for Parser<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parser({:?})", self.rest)
    }
}

fn describe(parser: &Parser<'_>) -> usize {
    parser.remaining()
}

fn main() {
    let parser = Parser { rest: "left over" };
    println!("remaining: {}", describe(&parser));
    println!("display: {parser}");
}
