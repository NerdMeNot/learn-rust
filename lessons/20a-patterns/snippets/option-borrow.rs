fn main() {
    let mut nick: Option<String> = Some(String::from("ana"));

    // match on a reference: `n` is a &String and nick stays whole
    if let Some(n) = &nick {
        println!("hello, {n}");
    }
    // as_ref() makes the same Option<&String> as a method call
    if let Some(n) = nick.as_ref() {
        println!("hello again, {n}");
    }
    // a mutable reference lets the arm change the String in place
    if let Some(n) = &mut nick {
        n.push_str("!!");
    }
    println!("nick is {nick:?}");
}
