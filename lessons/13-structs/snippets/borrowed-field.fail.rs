struct Excerpt {
    text: &str,
}

fn main() {
    let e = Excerpt {
        text: "call me Ishmael",
    };
    println!("{}", e.text);
}
