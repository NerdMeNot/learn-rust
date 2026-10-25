// One lifetime for both fields: the compiler must pick one that fits both.
struct Search<'a> {
    text: &'a str,
    query: &'a str,
}

impl<'a> Search<'a> {
    fn first_match(&self) -> Option<&'a str> {
        let at = self.text.find(self.query)?;
        Some(&self.text[at..at + self.query.len()])
    }
}

fn main() {
    let text = String::from("lifetimes are regions");
    let found;
    {
        let query = String::from("are");
        let search = Search {
            text: &text,
            query: &query,
        };
        found = search.first_match();
    }
    println!("found: {found:?}");
}
