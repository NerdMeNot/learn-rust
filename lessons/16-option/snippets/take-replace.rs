struct Mailbox {
    letter: Option<String>,
}

impl Mailbox {
    // take: hand the letter over and leave None behind.
    fn collect(&mut self) -> Option<String> {
        self.letter.take()
    }

    // replace: put a new letter in and get the old one back.
    fn deliver(&mut self, text: &str) -> Option<String> {
        self.letter.replace(String::from(text))
    }
}

fn main() {
    let mut mailbox = Mailbox { letter: None };
    println!("deliver 'hi': old = {:?}", mailbox.deliver("hi"));
    println!("deliver 'again': old = {:?}", mailbox.deliver("again"));
    println!("collect: {:?}", mailbox.collect());
    println!("collect again: {:?}", mailbox.collect());
    println!("now: {:?}", mailbox.letter);
}
