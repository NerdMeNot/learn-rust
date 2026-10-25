// returns a NEW String: the caller owns it
fn make_greeting(name: &str) -> String {
    format!("hello {name}")
}

// returns a piece of the input: no copy, borrowed from `text`
fn first_word(text: &str) -> &str {
    text.split(' ').next().unwrap_or("")
}

fn main() {
    let g = make_greeting("rust");
    let w = first_word(&g);
    println!("{g} / {w}");
}
