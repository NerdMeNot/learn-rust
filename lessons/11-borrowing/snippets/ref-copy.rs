fn show(text: &String) {
    println!("show: {text}");
}

fn main() {
    let name = String::from("Ada");
    let r = &name;

    // Passing r does not use it up: &String is Copy.
    show(r);
    show(r);

    let r2 = r; // copies the reference; r is still usable
    println!("r and r2 point at the same String: {}", std::ptr::eq(r, r2));

    // A String is not Copy: this moves it.
    let moved = name;
    println!("moved: {moved}");
}
