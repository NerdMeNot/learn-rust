fn tail<'a>(text: &'a str, skip: &'a str) -> &'a str {
    text.strip_prefix(skip).unwrap_or(text)
}

fn main() {
    let text = String::from("re:lifetimes");
    let result;
    {
        let skip = String::from("re:");
        result = tail(&text, &skip);
    }
    println!("tail: {result}");
}
