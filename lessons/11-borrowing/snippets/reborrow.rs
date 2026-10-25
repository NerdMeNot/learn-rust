fn main() {
    let name = String::from("Ada");
    let a: &String = &name;

    // *a is the String itself. &*a borrows it again: a new reference, same String.
    let b: &String = &*a;
    println!("b points at the same String as a: {}", std::ptr::eq(a, b));

    // *name is the str inside the String, so &*name is a &str. This is the
    // conversion the compiler does for you when it turns a &String into a &str.
    let text: &str = &*name;
    println!("text is the same bytes: {}", text.as_ptr() == name.as_ptr());
    println!("text: {text}, len {}", text.len());
}
