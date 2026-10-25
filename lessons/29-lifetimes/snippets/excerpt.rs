use std::mem::size_of;

struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let text = String::from("Call me Ishmael. Some years ago");
    let part = text.split('.').next().unwrap_or("");
    let excerpt = Excerpt { part };
    println!("text.len(): {}", text.len());
    println!("text.capacity(): {}", text.capacity());
    println!("excerpt.part: {:?}", excerpt.part);
    println!("excerpt.part.len(): {}", excerpt.part.len());
    println!("same address: {}", excerpt.part.as_ptr() == text.as_ptr());
    println!("size_of::<&str>(): {}", size_of::<&str>());
    println!("size_of::<Excerpt>(): {}", size_of::<Excerpt>());
}
