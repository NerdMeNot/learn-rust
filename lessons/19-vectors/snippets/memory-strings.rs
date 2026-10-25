use std::mem::size_of;

fn main() {
    let names = vec![String::from("ana"), String::from("bo")];

    println!("size_of::<String>(): {}", size_of::<String>());
    println!("names: len {}, capacity {}", names.len(), names.capacity());
    for (i, s) in names.iter().enumerate() {
        println!(
            "names[{i}]: \"{s}\" len {}, capacity {}",
            s.len(),
            s.capacity()
        );
    }
    println!(
        "the items are {} bytes apart in the list's buffer",
        names[1..].as_ptr() as usize - names.as_ptr() as usize
    );
    println!(
        "the first text is in a different block: {}",
        names[0].as_ptr() as usize != names.as_ptr() as usize
    );
}
