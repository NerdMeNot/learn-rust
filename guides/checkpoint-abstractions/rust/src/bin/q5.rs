struct Parser<'a> {
    text: &'a str,
}

fn main() {
    let parser;
    {
        let line = String::from("go north");
        parser = Parser { text: &line };
    }
    println!("{}", parser.text);
}
