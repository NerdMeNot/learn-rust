struct Inbox {
    unread: Vec<String>,
}

impl Inbox {
    /// Remove and return the oldest (first) message, or None if there are none.
    fn take_oldest(&mut self) -> Option<String> {
        todo!("check is_empty, then remove(0)")
    }

    /// Hand over every message at once, leaving the inbox empty but usable.
    fn take_all(&mut self) -> Vec<String> {
        todo!("std::mem::take the Vec out of self")
    }
}

fn main() {
    let mut inbox = Inbox {
        unread: vec![
            String::from("hi"),
            String::from("lunch?"),
            String::from("bye"),
        ],
    };

    // Some checks compare heap addresses: the text should move, not be copied.
    let hi_text = inbox.unread[0].as_ptr();
    let oldest = inbox.take_oldest().unwrap();
    assert_eq!(oldest, "hi");
    assert_eq!(
        oldest.as_ptr(),
        hi_text,
        "take_oldest should move, not clone"
    );
    assert_eq!(inbox.unread, ["lunch?", "bye"]);

    let lunch_text = inbox.unread[0].as_ptr();
    let all = inbox.take_all();
    assert_eq!(all, ["lunch?", "bye"]);
    assert_eq!(
        all[0].as_ptr(),
        lunch_text,
        "take_all should move, not clone"
    );
    assert!(
        inbox.unread.is_empty(),
        "take_all must leave the inbox empty"
    );

    // An empty inbox has nothing to give, and doesn't panic.
    assert_eq!(inbox.take_oldest(), None);
    assert!(inbox.take_all().is_empty());

    // And it still works afterwards.
    inbox.unread.push(String::from("new"));
    assert_eq!(inbox.take_oldest(), Some(String::from("new")));
    println!("all checks passed");
}
