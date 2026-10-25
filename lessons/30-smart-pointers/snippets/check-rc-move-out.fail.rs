use std::rc::Rc;

fn main() {
    let shared = Rc::new(String::from("x"));
    let owned: String = *shared;
    println!("{owned}");
}
