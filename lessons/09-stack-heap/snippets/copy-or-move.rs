fn main() {
    // A Copy value: the bits are copied, and both variables stay usable.
    let n = 7;
    let m = n;
    println!("n = {n}, m = {m}");

    // A String: only the header moves. The text stays where it was.
    let a = String::from("hello");
    let text_before = a.as_ptr();
    let b = a; // `a` can't be used after this line
    println!("b = {b}");
    println!("len {}, capacity {}", b.len(), b.capacity());
    println!("text stayed where it was: {}", b.as_ptr() == text_before);
}
