fn main() {
    let names = vec![String::from("ana"), String::from("bo"), String::from("cy")];

    let first: &String = names.iter().next().unwrap();
    // The item points at the String inside the Vec. It is not a copy.
    println!("points into the Vec: {}", std::ptr::eq(first, &names[0]));
    println!("a &String is {} bytes", size_of::<&String>());
    println!(
        "a String is {} bytes, plus its text on the heap",
        size_of::<String>()
    );
}
