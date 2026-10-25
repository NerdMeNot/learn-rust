fn main() {
    let a = vec![1, 2, 3];
    let mut b = a.clone(); // a new heap buffer with copies of the elements

    println!("different buffers: {}", a.as_ptr() != b.as_ptr());

    b.push(4); // changing b leaves a alone
    println!("a: {a:?}");
    println!("b: {b:?}");
    println!("a capacity {}, b capacity {}", a.capacity(), b.capacity());

    // Cloning a Vec<String> clones each String too: String's clone copies its text.
    let names = vec![String::from("ana"), String::from("bo")];
    let copy = names.clone();
    println!(
        "names[0] and copy[0] share text: {}",
        names[0].as_ptr() == copy[0].as_ptr()
    );
}
