struct Doc<'a> {
    text: String,
    first: &'a str,
}

fn main() {
    let text = String::from("hello world");
    let first = text.split(' ').next().unwrap();
    let doc = Doc { text, first };
    println!("{} / {}", doc.text, doc.first);
}
