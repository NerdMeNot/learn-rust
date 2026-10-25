// The frame of `greeting` is gone when it returns, but the text it hands back
// is not in the frame, so it is still there.
fn greeting() -> &'static str {
    "hello"
}

fn main() {
    let g = greeting();
    println!("g: {g}");

    // The variable `g` is a pointer and a length on the stack.
    println!("size of g: {} bytes", size_of_val(&g));
    // The text it points at is 5 bytes of the program's own data.
    println!("size of the text: {} bytes", size_of_val(g));
}
