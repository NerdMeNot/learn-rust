use std::any::type_name_of_val as type_of;

fn show(text: &str) {
    println!("show got {}: {text}", type_of(&text));
}

fn main() {
    let name = String::from("Ada");
    let r = &name;
    println!("r is {}", type_of(&r));

    // The function wants &str and r is &String. The compiler converts for us.
    show(r);

    // Same for a let with a written type.
    let text: &str = r;
    println!("text is {}", type_of(&text));
    println!("same text, no copy: {}", text.as_ptr() == name.as_ptr());

    // A &Vec<i32> becomes a &[i32] the same way.
    let v = vec![1, 2, 3];
    let slice: &[i32] = &v;
    println!("slice is {} with len {}", type_of(&slice), slice.len());
}
