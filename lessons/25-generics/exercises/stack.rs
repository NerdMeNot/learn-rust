// A stack of any type: the last item pushed is the first one popped.
use std::fmt::Display;

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        todo!("an empty stack")
    }

    fn push(&mut self, item: T) {
        todo!("add to the end of items")
    }

    fn pop(&mut self) -> Option<T> {
        todo!("take from the end of items")
    }

    fn peek(&self) -> Option<&T> {
        todo!("look at the end of items without taking it")
    }

    fn len(&self) -> usize {
        todo!("how many items")
    }

    // Looks like "[1, 2, 3]", bottom of the stack first.
    fn show(&self) -> String {
        todo!("join the items with \", \" inside brackets")
    }
}

// No Display, Clone or Debug: a Stack must still hold it.
struct Token {
    id: u32,
}

fn main() {
    let mut numbers = Stack::new();
    assert_eq!(numbers.pop(), None);
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers.peek(), Some(&3));
    assert_eq!(numbers.len(), 3);
    assert_eq!(numbers.show(), "[1, 2, 3]");
    assert_eq!(numbers.pop(), Some(3));
    assert_eq!(numbers.show(), "[1, 2]");
    for n in 3..=6 {
        numbers.push(n);
    }
    assert_eq!(numbers.len(), 6);
    assert_eq!(numbers.show(), "[1, 2, 3, 4, 5, 6]");

    let mut words = Stack::new();
    words.push(String::from("ana"));
    words.push(String::from("bo"));
    assert_eq!(words.show(), "[ana, bo]");
    assert_eq!(words.pop(), Some(String::from("bo")));

    let empty: Stack<char> = Stack::new();
    assert_eq!(empty.show(), "[]");

    let mut tokens = Stack::new();
    tokens.push(Token { id: 7 });
    assert_eq!(tokens.peek().map(|t| t.id), Some(7));
    let token = tokens.pop().unwrap();
    assert_eq!(token.id, 7);
    assert_eq!(tokens.len(), 0);
    println!("all checks passed");
}
