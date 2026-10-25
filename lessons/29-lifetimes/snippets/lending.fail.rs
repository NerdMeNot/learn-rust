struct Words {
    text: String,
}

impl Iterator for Words {
    type Item = &str; // would borrow from the Words that `next` was called on

    fn next(&mut self) -> Option<&str> {
        self.text.split(' ').next()
    }
}

fn main() {
    let words = Words {
        text: String::from("lifetimes are regions"),
    };
    for word in words {
        println!("word: {word}");
    }
}
