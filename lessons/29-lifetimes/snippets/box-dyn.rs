trait Describe {
    fn describe(&self) -> String;
}

struct Label<'a> {
    text: &'a str,
}

impl Describe for Label<'_> {
    fn describe(&self) -> String {
        format!("label {}", self.text)
    }
}

// `Box<dyn Describe>` would mean `Box<dyn Describe + 'static>`. This one borrows `text`.
fn boxed<'a>(text: &'a str) -> Box<dyn Describe + 'a> {
    Box::new(Label { text })
}

fn main() {
    let text = String::from("borrowed");
    let item = boxed(&text);
    println!("describe: {}", item.describe());
}
