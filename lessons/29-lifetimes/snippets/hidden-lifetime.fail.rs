struct Parser<'a> {
    rest: &'a str,
}

// `&Parser` has two lifetimes: the borrow of the parser and the `'a` inside it.
fn rest(parser: &Parser) -> &str {
    parser.rest
}

fn main() {
    let parser = Parser { rest: "left over" };
    println!("rest: {}", rest(&parser));
}
