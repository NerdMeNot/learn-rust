fn write_greeting(name: &str, out: &mut String) {
    out.push_str("hello ");
    out.push_str(name);
}

fn main() {
    let mut out = String::with_capacity(64);
    let start = out.as_ptr() as usize;
    for name in ["ana", "bo", "cy"] {
        out.clear();
        write_greeting(name, &mut out);
        println!("{out}");
    }
    println!("same buffer: {}", out.as_ptr() as usize == start);
    println!("capacity: {}", out.capacity());
}
