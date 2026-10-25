// Rule 2: one input lifetime is given to the output.
fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}
fn first_word_written<'a>(text: &'a str) -> &'a str {
    text.split(' ').next().unwrap_or("")
}

// Rule 1: every elided input gets its own lifetime. `skip` is not tied to the result.
fn tail<'a>(text: &'a str, skip: &str) -> &'a str {
    text.strip_prefix(skip).unwrap_or(text)
}

fn tail_written<'a, 'b>(text: &'a str, skip: &'b str) -> &'a str {
    text.strip_prefix(skip).unwrap_or(text)
}

struct Library {
    name: String,
}

impl Library {
    // Rule 3: with &self, the output gets the lifetime of self.
    fn label(&self, _prefix: &str) -> &str {
        &self.name
    }
    fn label_written<'a, 'b>(&'a self, _prefix: &'b str) -> &'a str {
        &self.name
    }
}

fn main() {
    println!("first_word: {}", first_word("hello rust"));
    println!("first_word_written: {}", first_word_written("hello rust"));

    let text = String::from("re:lifetimes");
    let result;
    {
        let skip = String::from("re:");
        result = tail(&text, &skip);
    } // `skip` is gone, `result` is still used below
    println!("tail: {result}");
    println!("tail_written: {}", tail_written("re:again", "re:"));

    let library = Library {
        name: String::from("city library"),
    };
    let label;
    {
        let prefix = String::from("main branch");
        label = library.label(&prefix);
    } // `prefix` is gone, `label` is still used below
    println!("label: {label}");
    println!("label_written: {}", library.label_written("x"));
}
