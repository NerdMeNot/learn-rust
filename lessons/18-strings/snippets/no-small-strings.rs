fn main() {
    // String::new() has not allocated anything yet.
    let empty = String::new();
    println!("String::new(): capacity {}", empty.capacity());

    // Two bytes of text, and they still go on the heap.
    let short = String::from("hi");
    println!("\"hi\": len {}, capacity {}", short.len(), short.capacity());

    // If the text were stored inside the String itself (as C++ does for short
    // strings), its address would fall inside the String's own 24 bytes.
    let header = &short as *const String as usize;
    let text = short.as_ptr() as usize;
    let inside = text >= header && text < header + size_of::<String>();
    println!("text stored inside the String: {inside}");
    println!("size_of::<String>(): {} bytes", size_of::<String>());
}
