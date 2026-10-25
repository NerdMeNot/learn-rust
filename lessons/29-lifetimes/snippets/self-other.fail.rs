struct Library {
    name: String,
}

impl Library {
    // Rule 3 ties the result to `self`, but this returns `other`.
    fn pick(&self, other: &str) -> &str {
        other
    }
}

fn main() {
    let library = Library {
        name: String::from("city library"),
    };
    println!("{}: {}", library.name, library.pick("elsewhere"));
}
