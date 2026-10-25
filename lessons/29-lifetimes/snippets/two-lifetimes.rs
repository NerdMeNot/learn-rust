// Two lifetimes: the text and the query can end at different times.
struct Search<'t, 'q> {
    text: &'t str,
    query: &'q str,
}

impl<'t, 'q> Search<'t, 'q> {
    // The match is a piece of the text, so it carries 't and not 'q.
    fn first_match(&self) -> Option<&'t str> {
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
    } // `query` and `search` are gone, `found` is not
    println!("found: {found:?}");
}
