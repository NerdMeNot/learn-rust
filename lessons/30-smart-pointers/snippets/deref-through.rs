use std::rc::Rc;

fn shout(text: &str) -> String {
    text.to_uppercase()
}

fn main() {
    let boxed = Box::new(String::from("boxed"));
    let shared = Rc::new(String::from("shared"));

    // methods of the String inside are reachable directly
    println!("len: {} and {}", boxed.len(), shared.len());

    // &Box<String> and &Rc<String> become &String, then &str
    println!("{} and {}", shout(&boxed), shout(&shared));

    // * steps through one pointer by hand
    let inner: &String = &*shared;
    println!("inner: {inner}");
}
