use std::mem::size_of;

fn length(s: &str) -> usize {
    s.len()
}

fn total(xs: &[i32]) -> i32 {
    xs.iter().sum()
}

fn main() {
    let owned = String::from("hello");
    let text: &str = &owned; // &String becomes &str: String implements Deref<Target = str>
    println!("length(&owned): {}", length(&owned));
    println!("same bytes: {}", owned.as_ptr() == text.as_ptr());
    println!(
        "String is {} bytes, &str is {}",
        size_of::<String>(),
        size_of::<&str>()
    );
    println!("owned.len() {}, text.len() {}", owned.len(), text.len());
    println!("capacity (String only): {}", owned.capacity());

    let v = vec![1, 2, 3];
    println!("total(&v): {}", total(&v)); // &Vec<i32> becomes &[i32]
    println!("method through Deref: {:?}", v.first()); // first() is a slice method
}
