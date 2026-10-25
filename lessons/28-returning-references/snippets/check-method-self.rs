struct Text(String);

impl Text {
    // two references in, but a method with `&self`: the result borrows from `self`
    fn before(&self, sep: &str) -> &str {
        self.0.split(sep).next().unwrap_or("")
    }
}

fn main() {
    let t = Text(String::from("a,b"));
    let r;
    {
        let sep = String::from(",");
        r = t.before(&sep); // `sep` is dropped at the closing brace; `r` is still fine
    }
    assert_eq!(r, "a");
}
